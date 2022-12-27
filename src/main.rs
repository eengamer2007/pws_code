use std::fs::File;
use flac::StreamReader;
use plotters::prelude::*;

mod fft;

const DIV: u32 = 2;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    match StreamReader::<File>::from_file("/home/tdjong/Downloads/nicotine/witch_doctor.flac") {
        Ok(mut stream) => {
            // get stream info
            let info = stream.info();
            println!("{info:?}");
            // make plotters plot
            println!("make file");
            let name = format!("out/test{DIV}.png");
            File::create(&name)?;
            let root = BitMapBackend::new(&name, ((info.total_samples as u32 + 40) / 10_u32.pow(DIV), 140))
                .into_drawing_area();
            println!("fill");
            root.fill(&WHITE)?;
            println!("start chart");
            let mut chart = ChartBuilder::on(&root)
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(0.0f64..(info.total_samples as f64), -1.0f64..1.0f64)?;
                

            println!("set mesh");
            chart.configure_mesh().draw()?;


            println!("start plot");
            chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f64 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))?
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));


            println!("make vec");
            let mut data: Vec<f64> = vec!();
            // iterate over all samples
            for sample in stream.iter::<i16>() {
                data.push(sample as f64 / std::i16::MAX as f64);
            }
            println!("fft");
            println!("{:?}", fft::fft(&data).await);

            println!("plot vec");
            chart
            .draw_series(LineSeries::new(
                (0..(data.len())).map(|x| (x as f64, data[x as usize])),
                &RED,)
            )?
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

            println!("save plot");
            root.present()?;
        },
        Err(error) => panic!("{error:?}"),
    }
    Ok(())
}

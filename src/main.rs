use flac::StreamReader;
use num::complex::ComplexFloat;
use plotters::prelude::*;
use std::fs::File;

use std::io::Write;
use std::time::Instant;

//use job-sys;

mod fft;
use fft::interpolate;

// function that i do not remember why it exist but it breaks when removed
fn flush() {
    std::io::stdout().flush().unwrap();
}

// function used to save graphs
fn save_graph(plot1: &[f64], plot2: &[f64], fname: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    // make the drawing area
    let root = SVGBackend::new(fname, ((plot1.len() + 40) as u32, 500)).into_drawing_area();
    let (upper, lower) = root.split_vertically(200);
    // fill drawing area with white
    root.fill(&WHITE)?;

    upper.titled(title, ("sans-serif", 20, &BLACK).into_text_style(&root))?;

    // get max and min value
    let mut max: f64 = 0.;
    let mut min: f64 = 0.;
    for i in plot1.iter() {
        if i > &max {
            max = *i
        } else if i < &min {
            min = *i
        };
    }

    println!(" min: {min}, max: {max}");

    // set the margin, label size and make the grid and obtain grid
    let mut chart1 = ChartBuilder::on(&upper)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(55)
        .build_cartesian_2d(0.0f64..(plot1.len() as f64), min..max)?;

    let mut max: f64 = 0.;
    let mut min: f64 = 0.;
    for i in plot2.iter() {
        if i > &max {
            max = *i
        } else if i < &min {
            min = *i
        };
    }

    let mut chart2 = ChartBuilder::on(&lower)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(55)
        .build_cartesian_2d(0.0f64..(plot2.len() as f64), min..max)?;

    // configure the mesg
    chart1.configure_mesh().draw()?;
    chart2.configure_mesh().draw()?;

    // draw the graph
    chart1
        .draw_series(LineSeries::new(
            (0..(plot1.len())).map(|x| (x as f64, plot1[x as usize])),
            &RED,
        ))?
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 1, y)], &RED));

    chart2
        .draw_series(LineSeries::new(
            (0..(plot2.len())).map(|x| (x as f64, plot2[x as usize])),
            &RED,
        ))?
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 1, y)], &RED));

    // write the graph to file
    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    let size = 2usize.pow(14);

    let r_min = 0;
    let r_max = r_min + size;

    let range = r_min..r_max;

    // collect data from song
    let (data, sample_rate): (Vec<f64>, u32) = match StreamReader::<File>::from_file("in/witch_doctor.flac") {
        Ok(mut stream) => {
            // get stream info
            let info = stream.info();
            println!("{info:?}");
            print!("make vec");
            flush();
            
            let data = stream
                .iter::<i16>()
                .map(|x| x as f64 / std::i16::MAX as f64)
                .collect();

            (data, info.sample_rate)
        }
        Err(error) => panic!("{error:?}"),
    };

    println!(": {:?}", start.elapsed());
    print!("fft");
    flush();
    let used_data = interpolate(&data[r_min..(size/2)]);
    println!(" {}", used_data.len());
    let mut fft_res = fft::fft(&used_data);
    
    // remove the DC component from the music
    fft_res[0] = num::Complex::new(0.,0.);

    println!(": {:?}", start.elapsed());
    print!("save plot");
    flush();
    let amp = fft_res.iter().map(|x| x.abs() as f64).collect::<Vec<f64>>();
    save_graph(
        &(amp[0..(size/2)]), &used_data,
        "./out/fourier.svg", &format!("{}: {:?}", r_max - r_min, (r_max - r_min) as f64 / sample_rate as f64)
    )?;
    println!(": {:?}", start.elapsed());

    for (i, x) in amp[0..(size/2)].iter().enumerate() {
        println!("{i:04} : {x:.1}");
    }
    Ok(())
}

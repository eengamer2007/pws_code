use num::complex::Complex;
use core::f64::consts::PI;

fn inner_fft(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();
    if n <= 1 {
        return x.to_owned();
    }
    let iter1 = x.iter().step_by(2).cloned().collect::<Vec<_>>();
    let iter2 = x.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>();
    let even = inner_fft(&iter1);
    let odd= inner_fft(&iter2);
    let mut temp = vec![Complex::new(0.0_f64, 0.0_f64); n];
    for k in 0..(n / 2) {
        temp[k] = even[k]
            + (Complex::new(0.0_f64, -2.0_f64) * PI * (k as f64) / (n as f64)).exp() * odd[k];
        temp[k + n / 2] = even[k]
            - (Complex::new(0.0_f64, -2.0_f64) * PI * (k as f64) / (n as f64)).exp() * odd[k];
    }
    temp
}

pub fn fft(input: &[f64]) -> Vec<Complex<f64>> {
    inner_fft(
        &(input.iter().map(|x| Complex::new(*x, 0.)).collect::<Vec<Complex<f64>>>())
    ).to_vec()
}

pub fn pad(input: &[f64], amount: Option<u32>) -> Result<Vec<f64>, ()> {
    if let Some(x) = amount {
        println!("{}", x);
    }
    Ok(vec!())
}

pub fn interpolate(input: &[f64]) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::with_capacity(input.len() * 2);
    for i in 0..(input.len() - 1) {
        output.push(input[i]);
        output.push((input[i] + input[i+1]) / 2.);
    }
    output
}

pub fn interpolate_zero(input: &[f64]) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::with_capacity(input.len() * 2);
    for i in input.iter() {
        output.push(*i);
        output.push(0.);
    }
    output
}
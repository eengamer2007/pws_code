use rustfft::num_complex::Complex;
use core::f64::consts::PI;

use async_recursion::async_recursion    ;

fn sync_fft() {

}

pub async fn fft(input: &[f64]) -> Vec<Complex<f64>> {
    fft_complex(
        &(input.iter().enumerate().map(|x| Complex::new(x.0 as f64, *x.1)).collect::<Vec<Complex<f64>>>())
    ).await.to_vec()
}

#[async_recursion(?Send)]
async fn fft_complex(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();
    if n <= 1 {
        return x.to_owned();
    }
    let even = fft_complex(&x.iter().step_by(2).cloned().collect::<Vec<_>>()).await;
    let odd  = fft_complex(&x.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>()).await;

    let mut temp = vec![Complex::new(0.0_f64, 0.0_f64); n];
    for k in 0..(n / 2) {
        temp[k] = even[k]
            + (Complex::new(0.0_f64, -2.0_f64) * PI * (k as f64) / (n as f64)).exp() * odd[k];
        temp[k + n / 2] = even[k]
            - (Complex::new(0.0_f64, -2.0_f64) * PI * (k as f64) / (n as f64)).exp() * odd[k];
    }
    temp
}

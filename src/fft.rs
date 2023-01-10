use num::complex::Complex;
use core::f64::consts::PI;

use async_recursion::async_recursion    ;

fn sync_fft() {

}

pub async fn fft(input: &[f64]) -> Vec<Complex<f64>> {
    fft_complex(
        //&(input.iter().enumerate().map(|x| Complex::new(x.0 as f64, *x.1)).collect::<Vec<Complex<f64>>>())
        &(input.iter().map(|x| Complex::new(*x, 0.)).collect::<Vec<Complex<f64>>>())
    ).await.to_vec()
}

#[async_recursion(?Send)]
async fn fft_complex(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();
    if n <= 1 {
        return x.to_owned();
    }
    let iter1 = x.iter().step_by(2).cloned().collect::<Vec<_>>();
    let iter2 = x.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>();
    let even_f = fft_complex(&iter1);
    let odd_f  = fft_complex(&iter2);
    let even = even_f.await;
    let odd = odd_f.await;
    let mut temp = vec![Complex::new(0.0_f64, 0.0_f64); n];
    for k in 0..(n / 2) {
        temp[k] = even[k]
            + (Complex::new(0.0_f64, -2.0_f64) * PI * (k as f64) / (n as f64)).exp() * odd[k];
        temp[k + n / 2] = even[k]
            - (Complex::new(0.0_f64, -2.0_f64) * PI * (k as f64) / (n as f64)).exp() * odd[k];
    }
    temp
}

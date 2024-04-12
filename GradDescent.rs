extern crate nalgebra as na;

use na::{DVector};

fn main() {
    let x = DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
    let y = DVector::from_vec(vec![3.0, 5.0, 7.0, 9.0, 13.0, 13.0, 17.0, 19.0, 21.0, 25.0, 27.0, 29.0]);
    
    let mut slope = 0.0;
    let mut intercept = 0.0;

    let lr = 0.01;

    let iters = 1000;
    
    for _ in 0..iters {
        let y_pred = &x * slope + &DVector::repeat(x.len(), intercept);

        let error = &y_pred - &y;

        let grad = 2.0 * x.dot(&error) / 12.0;
        let intercept_grad = 2.0 * error.sum() / 12.0;

        slope -= lr * grad;
        intercept -= lr * intercept_grad;
        
        
    }
    println!("Slope: {}", slope);
    println!("Intercept: {}", intercept);
}

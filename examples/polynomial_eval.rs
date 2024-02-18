#![allow(dead_code)]


// Define a function to evaluate a polynomial using Horner's algorithm
fn horner(coefficients: &[f64], x: f64) -> f64 {
    let mut result = coefficients[0];  // The first coefficient is the highest power
    coefficients.iter().skip(1).rev().for_each(|&coefficient| {
        result = result * x + coefficient;  // Multiply the result by x and add the next coefficient
    });
    result
}


// I love what Volker Strassen did in the subject of matrix multiplication
// But this? It doesn't seem to be a good idea to evaluate a polynomial this way...
fn volker_strassen(coefficients: &[f64], x: f64) -> f64 {
    let mut result = 0.0;
    // todo: Check if the coefficients are in the correct order
    // coefficients.iter().enumerate().rev().for_each(|(i, &coefficient)| {
    coefficients.iter().enumerate().for_each(|(i, &coefficient)| {
        result += coefficient * x.powi(i as i32);
    });
    result
}


fn eval_polynomial(method: fn(&[f64], f64) -> f64, coefficients: &[f64], min: f64, max: f64, step: f64) {
    // todo: Compare the performance of the two methods...
    // I bet to myself that the second method is faster... Or that's what I think...
    // I will test it later...

    let mut x = min;
    while x <= max {
        println!("f({:.2}) = {:.4}", x, method(coefficients, x));
        x += step;  // Step size
    }

    // (0..((max - min) / step) as u32 + 1).map(|i| min + i as f64 * step)
    //     .for_each(|x| println!("f({x:.2}) = {:.4}", horner(&coefficients, x))
    // );

}


fn main() {
    // The polynomial is represented as a vector of its coefficients
    // * coeficients.n => x^(coeficients.len() - n - 1)
    let coefficients = vec![
        2.0,
        -1.0,
        0.0
    ];

    let min: f64 = -5.0;
    let max: f64 = 5.0;
    let step: f64 = 0.2;

    eval_polynomial(horner, &coefficients, min, max, step);
    eval_polynomial(volker_strassen, &coefficients, min, max, step);
}

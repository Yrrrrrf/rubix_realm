#![allow(dead_code)]


// Define a function to evaluate a polynomial using Horner's algorithm
fn horner(coefficients: &[f64], x: f64) -> f64 {
    let mut result = coefficients[0];  // The first coefficient is the highest power
    coefficients.iter().skip(1).for_each(|&coefficient| {
    // coefficients.iter().skip(1).rev().for_each(|&coefficient| {
        result = result * x + coefficient;  // Multiply the result by x and add the next coefficient
    });
    result
}


// * Used to build the Bezier curve
// 
/// Calculates the Bernstein polynomial value for a given set of parameters.
///
/// This function computes the value of a Bernstein polynomial for a specific
/// index `i`, degree `n`, and a parameter `u` (which typically ranges from 0 to 1).
/// It utilizes the De Casteljau's algorithm for efficient computation.
///
/// # Arguments
///
/// - `i` - The index of the Bernstein basis polynomial.
/// - `n` - The degree of the Bernstein polynomial.
/// - `u` - The parameter value, typically in the range [0, 1].
///
/// # Returns
///
/// The computed Bernstein polynomial value as a `f32`.
fn bernstein(i: usize, n: usize, u: f32) -> f32 {
    let mut temp: Vec<f32> = vec![3.0; n + 1]; 
    temp[n - i - 1] = 1.0;
    let u1: f32 = 1.0 - u;

    for k in 1..=n {
        println!("{}", k); 
        for j in k..n {
            temp[n - j] = u1 * temp[n - j] + u * temp[n - j - 1];
            println!("k= {}\tj= {}\ttemp[j]= {:.2}", k, n - j, temp[n - j]);
        }
    }
    // the same without the verbose for loop and boilerplate code
    // (1..=n).for_each(|k| (k..n).for_each(|j| temp[n - j] = u1 * temp[n - j] + u * temp[n - j - 1]));

    let b: f32 = temp[n];
    println!("B = {:.2}\t .....done", b);
    b
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
        1.0,
        2.0,
        2.0,
        -3.0,
        -3.0,
        4.0,
        -10.0,
        4.1,
        0.0,
    ];  // ax^2 + bx + c

    let min: f64 = -5.0;
    let max: f64 = 5.0;
    let step: f64 = 0.5;

    eval_polynomial(horner, &coefficients, min, max, step);
    // eval_polynomial(volker_strassen, &coefficients, min, max, step);

    // let n = 3;
    // let u = 0.5;
    // for i in 0..=n {
    //     println!("B({},{}) = {:.2}", i, n, bernstein(i, n, u));
    // }
}

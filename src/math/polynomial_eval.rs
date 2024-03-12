// #![allow(dead_code)]


// Define a function to evaluate a polynomial using Horner's algorithm
pub fn horner(coefficients: &[f64], x: f64) -> f64 {
    let mut result = coefficients[0];  // The first coefficient is the highest power
    coefficients.iter().skip(1).for_each(|&coefficient| result = result * x + coefficient);
    result
    // do the same as above with a list comprehension
}


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
pub fn bernstein(i: usize, n: usize, u: f64) -> f64 {
    let mut temp = vec![3.0; n + 1]; 
    temp[n - i - 1] = 1.0;
    let u1: f64 = 1.0 - u;

    for k in 1..=n {
        println!("{}", k); 
        for j in k..n {
            temp[n - j] = u1 * temp[n - j] + u * temp[n - j - 1];
            println!("k= {}\tj= {}\ttemp[j]= {:.2}", k, n - j, temp[n - j]);
        }
    }
    // the same without the verbose for loop and boilerplate code
    // (1..=n).for_each(|k| (k..n).for_each(|j| temp[n - j] = u1 * temp[n - j] + u * temp[n - j - 1]));

    let b = temp[n];
    println!("B = {:.2}\t ...done", b);
    b
}

// todo: implement the casteljau algorithm
pub fn casteljau() {
    todo!("Implement the De Casteljau's algorithm");
}

pub fn polynomial_coefficients() {
    todo!("Implement the polynomial_coefficients fn that represents a B^3 curve");
}

// todo: implement the bezier_curve function
pub fn bezier_curve(points: Vec<(f64, f64)>) {
    todo!("Impl the iteration of the Bezir curve over the range [0, 1] for the given points");
}


pub fn eval_polynomial(method: fn(&[f64], f64) -> f64, coefficients: &[f64], min: f64, max: f64, step: f64) {
    (0..((max - min) / step) as u32 + 1).map(|i| min + i as f64 * step)
        .for_each(|x| println!("f({x:.2}) = {:.4}", method(&coefficients, x))
    );
}

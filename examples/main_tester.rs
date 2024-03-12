//! This handles all possible unused warnings (but it's a little overkill, so you can remove it if you want to see all the warnings)
#![allow(unused)]
// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]


use rubix_realm::{
    init_window,
    math::Matrix,
};




fn main() {
    // if let Err(e) =  pollster::block_on(init_window()) {eprintln!("Error: {}", e);}

    // test_polynomial();
    matrix_test();
}

fn test_polynomial() {
    use rubix_realm::math::polynomial_eval::*;
    // The polynomial is represented as a vector of its coefficients
    // * coeficients.n => x^(coeficients.len() - n - 1)
    let coefficients = vec![
        -0.2,
        1.0,
        2.0,
        -1.0,
    ];  // ax^2 + bx + c
    let min: f64 = -2.0;
    let max: f64 = 2.0;
    let step: f64 = 0.1;

    eval_polynomial(horner, &coefficients, min, max, step);
    // eval_polynomial(volker_strassen, &coefficients, min, max, step);


}

pub fn matrix_test() {
    // let a = Matrix::new(vec![
    //     vec![1.0, 2.0, 2.0], 
    //     vec![1.0, 2.0, 2.0], 
    //     vec![1.0, 2.0, 2.0], 
    //     vec![1.0, 2.0, 2.0], 
    //     vec![1.0, 2.0, 2.0], 
    //     vec![3.0, 4.0, 4.0]
    //     ]);
    // let b = Matrix::new(vec![
    //     vec![5.0, 6.0], 
    //     vec![5.0, 6.0], 
    //     vec![7.0, 8.0]
    //     ]);

    let a = Matrix::new(vec![
        vec![1.0, 2.0], 
        vec![3.0, 4.0]
        ]);
    let b = Matrix::new(vec![
        vec![5.0, 6.0], 
        vec![7.0, 8.0]
        ]);

    let c = a.clone() * b.clone(); // Demonstrating multiplication

    // println!("Matrix A:\n{}", a);
    // println!("Matrix B:\n{}", b);
    // println!("Matrix A + B:\n{}", a + b);

    println!("Matrix A * B:\n{}", a.clone()*3.0);
    println!("Matrix A * B:\n{}", 3.0*a);


}

// #![allow(unused)]


use nannou::prelude::*;
use nannou::geom::Point2;

use crate::Model;

use super::matrix::*;


pub fn bezier_surface_at(u: f32, v: f32, control_points: &Vec<Vec<Point2>>) -> Point2 {
    // Validate the input
    assert!(u >= 0.0 && u <= 1.0, "The parameter `u` must be between 0 and 1");
    assert!(v >= 0.0 && v <= 1.0, "The parameter `v` must be between 0 and 1");
    // assert!(control_points.len() == 4 && control_points[0].len() == 4, "The Cubic Bezier surface requires a 4x4 grid of control points");

    // Create a matrix with the u and v parameters
    let u_vec = Matrix::new(vec![
        vec![u.powi(3) as f64, u.powi(2) as f64, u as f64, 1.0]
    ]);
    let v_vec = Matrix::new(vec![
        vec![v.powi(3) as f64, v.powi(2) as f64, v as f64, 1.0]
    ]);

    // Create the bezier matrix
    let bezier_matrix_u = u_vec * b3_matrix();
    let bezier_matrix_v = v_vec * b3_matrix();

    // Multiply the bezier matrix by the control points matrix
    let x_vals: Vec<Vec<f64>> = control_points.iter().map(|row| {
        row.iter().map(|point| point.x as f64).collect()
    }).collect();
    let y_vals: Vec<Vec<f64>> = control_points.iter().map(|row| {
        row.iter().map(|point| point.y as f64).collect()
    }).collect();

    // Convert control point matrices to Matrix type
    let x_matrix = Matrix::new(x_vals);
    let y_matrix = Matrix::new(y_vals);

    // Calculate the x and y values
    let x = bezier_matrix_u.clone() * x_matrix * bezier_matrix_v.transpose();
    let y = bezier_matrix_u * y_matrix * bezier_matrix_v.transpose();

    Point2::new(x.data[0][0] as f32, y.data[0][0] as f32)  // Return the point [x, y]
}

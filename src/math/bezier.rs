// #![allow(unused)]

use nannou::geom::Point2;
use nannou::prelude::*;

use crate::Model;

use super::matrix::*;

pub fn bezier(draw: &nannou::draw::Draw, model: &Model, control_points: &Vec<Point2>) {
    let fragments: usize =  // Number of fragments to draw the curve
    20
    // 100
    ;

    (0..=fragments)
        .map(|t| t as f32 / fragments as f32)
        .for_each(|t| {
            draw.ellipse()
                .xy(bezier_at(t, &control_points))
                .radius(1.0)
                .color(GREENYELLOW);
        });

    draw.ellipse()
        .xy(bezier_at(0.0, &control_points))
        .radius(10.0)
        .color(GREENYELLOW); // draw the first point
}

// * where `t` is the time variable that goes from 0 to 1
fn bezier_at(t: f32, points: &Vec<Point2>) -> Point2 {
    // Validate the input
    assert!(
        t >= 0.0 && t <= 1.0,
        "The time variable `t` must be between 0 and 1"
    );
    assert!(
        points.len() == 4,
        "The Cubic Bezier curve requires 4 control points"
    );

    // Create a matrix with the control points
    // let t_vec =

    let bezier_matrix = Matrix::new(vec![
        // (3..=0).map(|n| t.powi(n as i32) as f64).collect::<Vec<f64>>()
        vec![t.powi(3) as f64, t.powi(2) as f64, t as f64, 1.0],
    ]) * b3_matrix(); // Multiply the t vector by the bezier matrix

    // multiply the bezier matrix by the control points matrix
    let x_val: Vec<f64> = points.iter().map(|point| point.x as f64).collect();
    let y_val: Vec<f64> = points.iter().map(|point| point.y as f64).collect();

    // Calculate the x and y values
    let x = bezier_matrix.clone() * Matrix::new(vec![x_val]).transpose();
    let y = bezier_matrix.clone() * Matrix::new(vec![y_val]).transpose();

    Point2::new(x.data[0][0] as f32, y.data[0][0] as f32) // Return the point [x, y
}

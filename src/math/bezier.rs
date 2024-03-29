// #![allow(unused)]


use nannou::prelude::*;
use nannou::geom::Point2;

use crate::Model;

use super::matrix::*;

pub fn bezier(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();  // Create a new draw object

    // Define the control points of the Bezier curve
    let control_points = vec![
        Point2::new(-200.0,   0.0),  // Initial point
        Point2::new(-100.0, 200.0),  // Control point 1
        Point2::new( 100.0, 200.0),  // Control point 2
        Point2::new( 200.0,   0.0),  // Final point
    ];  // Points for a Cubic Bezier curve (most common)


    draw_points(&draw, &control_points);  // Draw the control points
    draw_conn_lines(&draw, &control_points);

    let t = 0.5;  // Time variable for the Bezier curve
    bezier_matrix_form(t, &control_points);

}




fn draw_points(draw: &Draw, points: &Vec<Point2>) {
    points.iter().for_each(|point| {draw.ellipse().xy(*point).radius(5.0).color(RED);});
}

fn draw_conn_lines(draw: &Draw, points: &Vec<Point2>) {
    (0..points.len()-1).for_each(|i| {
        draw.line().start(points[i]).end(points[i+1]).weight(2.0).color(WHITE);
    });
}


// * where `t` is the time variable that goes from 0 to 1
fn bezier_matrix_form(t: f32, points: &Vec<Point2>) -> Point2 {
    // Validate the input
    assert!(t >= 0.0 && t <= 1.0, "The time variable `t` must be between 0 and 1");
    assert!(points.len() == 4, "The Cubic Bezier curve requires 4 control points");


    // Create a matrix with the control points
    let t_vec = Matrix::new(vec![
        vec![t.powi(3) as f64, t.powi(2) as f64, t as f64, 1.0]
    ]);
    // todo: Look for a way to create the t_vec matrix without hardcoding the values (use a loop or iterator)
    // Check this code snippet: (fix it and use it to create the t_vec matrix)
    // let t_vec = Matrix::new(
    //     (3..=0).map(|i| t.powi(i) as f64).collect::<Vec<f64>()
    // );

    let mat_form: Matrix = b3_matrix();

    let bezier_matrix = t_vec * mat_form;

    // multiply the bezier matrix by the control points matrix
    // get the x values of the points
    let x_val: Vec<f64> = points.iter().map(|point| point.x as f64).collect();
    // get the y values of the points
    let y_val: Vec<f64> = points.iter().map(|point| point.y as f64).collect();

    let x = bezier_matrix.clone() * Matrix::new(vec![x_val]).transpose();
    let y = bezier_matrix.clone() * Matrix::new(vec![y_val]).transpose();



    println!("Bezier matrix: {:?}", bezier_matrix);
    println!("x: {:?}", x);
    println!("y: {:?}", y);

    Point2::new(0.0, 0.0)
}
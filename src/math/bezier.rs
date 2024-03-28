// #![allow(unused)]


use nannou::prelude::*;
use nannou::geom::Point2;

use crate::Model;

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

    

}




fn draw_points(draw: &Draw, points: &Vec<Point2>) {
    points.iter().for_each(|point| {draw.ellipse().xy(*point).radius(5.0).color(RED);});
}

fn draw_conn_lines(draw: &Draw, points: &Vec<Point2>) {
    (0..points.len()-1).for_each(|i| {
        draw.line().start(points[i]).end(points[i+1]).weight(2.0).color(WHITE);
    });
}


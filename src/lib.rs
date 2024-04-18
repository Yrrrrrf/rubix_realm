#![allow(unused)]


// ? Library metadata -------------------------------------------------------------------------
use nannou::{draw, event::{ElementState, KeyboardInput}, geom::point, image, prelude::*, winit::window::Icon};

pub mod n_test;  // & nannou: library for creative coding in Rust
// pub mod util;  // for now just contains `lorem ipsum` text generator
pub mod math;
pub mod geometry;

use math::bezier::*;

use crate::math::bezier;


// ? General library functionalities ----------------------------------------------------------


pub fn init_window() {
    nannou::app(model)
        // .update(update)  // Update the model here...<every frame>
        .event(event)  // Handle events
        .simple_window(view)  // Draw the model
        .run();
}




pub struct Model {
    // line_start_pos:Point2,  // The starting position of the line
    b_lines: Vec<(Point2, Point2)>, // Each line is represented as a pair of start and end points
    points: Vec<Point2>, // Stores the points added by mouse clicks
}
fn model(app: &App) -> Model {
    // * Set the window properties
    app.main_window().set_min_inner_size_points(Some((720.0, 480.0)));
    app.main_window().set_inner_size_pixels(1920, 1080);
    // app.main_window().set_inner_size_pixels(1080, 720);
    app.main_window().set_title("Rubix Realm");
    app.main_window().set_window_icon(
        Some(load_icon("./assets/icons/rubik.png").expect("Failed to load window icon"))
    );

    // * Initialize the model
    Model {
        // line_start_pos: app.window_rect().xy(),
        b_lines: Vec::new(),  // Initialize the lines vector
        points: Vec::new(), // Initialize the points vector
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _app.main_window().set_title(&format!("{:.0} fps", _app.fps()));
}


fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {simple: Some(MousePressed(MouseButton::Left)), ..
        } => {
            handle_mouse_event(app, model);
        },
        Event::WindowEvent {simple: Some(WindowEvent::KeyPressed(key_code)), ..} => {
            handle_key_event(model, key_code);
        },
        _ => {}
    }
}


/// View function that 
// This fn is excecuted every frame
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK); // Clear the background to remove previous drawings

    // Draw the Bezier curve
    // Only if there are 4 points.. Or more.
    // Like 4 + 3n points
    // This to draw a continuous curve
    // 4 for the first curve
    // 3 for the next curve (last point of the previous curve is the first point of the next curve)
        // 1 last point
        // +2 control points
        // +1 last point (will be used as the first point of the next curve)
    if model.points.len() >= 4 {
        let mut start_index = 0;
        let mut end_index = 3;

        while end_index < model.points.len() {
            // println!("Start index: {}, End index: {}", start_index, end_index);  // Debugging print statement
            let control_points = &model.points[start_index..=end_index].to_vec();
            bezier(&draw, &model, control_points);
            start_index += 3;
            end_index += 3;
        }
    }
// 4 + 3n points
// using some modulo operation:
// 4 + 3n points



    // Draw the lines
    model.b_lines.iter().for_each(|(start, end)| {
        draw.line().start(*start).end(*end).weight(0.5).color(WHITE);
    });

    // Draw the points
    model.points.iter().for_each(|point| {draw.ellipse().xy(*point).radius(5.0).color(BLUEVIOLET);});
    
    draw.to_frame(app, &frame).unwrap();  //* Draw all the elements above to the frame...
}

// fn draw_point(draw: &nannou::draw::Draw, model: &Model, frame: &Frame, point: Point2) {
//     println!("Drawing point: {:?}", point);  // Debugging print statement
//     draw.ellipse().xy(point).radius(5.0).color(BLUEVIOLET);
// }

fn draw_conn_lines(draw: &draw::Draw, points: &Vec<Point2>) {
    (0..points.len()-1).for_each(|i| {
        draw.line().start(points[i]).end(points[i+1]).weight(0.05).color(WHITE);
    });
}














fn load_icon(icon_path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    let image_data = image::open(icon_path)?.to_rgba8();
    let (width, height) = image_data.dimensions();
    Ok(Icon::from_rgba(image_data.into_raw(), width, height).unwrap())
}


fn handle_mouse_event(app: &App, model: &mut Model) {
    let mouse_pos = app.mouse.position();
    println!("Mouse position: {:?}", mouse_pos);  // Debugging print statement
    model.points.push(mouse_pos);  // Add the point to the points vector
}

fn handle_key_event(model: &mut Model, key_code: Key) {
    match key_code {
        Key::C => {
            model.points.clear();
            println!("Cleared all points");
        }
        Key::Escape => {
            println!("Exiting the application...");
            // std::process::exit(0);
        }
        _ => {
            println!("No action for key: {:?}", key_code);
        }
    }
}

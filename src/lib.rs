#![allow(unused)]


// ? Library metadata -------------------------------------------------------------------------
use nannou::{draw, event::{ElementState, KeyboardInput}, geom::point, image, prelude::*, winit::window::Icon};

pub mod n_test;  // & nannou: library for creative coding in Rust
// pub mod util;  // for now just contains `lorem ipsum` text generator
pub mod math;
pub mod geometry;

use math::bezier::*;
use math::surface::*;


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
    control_points: Vec<Vec<(f32, f32)>>,  // Stores the control points for the Bezier curve
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


    let scale = 300.0;
    let mut control_points = vec![vec![
        (-1.0, -1.0),  // 0 
        (-1.0,  1.0),  // 1
        ( 1.0,  1.0),  // 3
        ( 1.0, -1.0),  // 2
    ]];
    // now scale the control points
    control_points.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|(x, y)| {
            *x *= scale;
            *y *= scale;
        });
    });

    // * Initialize the model
    Model {
        // line_start_pos: app.window_rect().xy(),
        b_lines: Vec::new(),  // Initialize the lines vector
        points: Vec::new(), // Initialize the points vector
        // control_points: Vec::new(),  // Initialize the control points vector
        control_points,
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
            handle_key_event(app, model, key_code);
        },
        _ => {}
    }
}


/// View function that 
// This fn is excecuted every frame
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK); // Clear the background to remove previous drawings

    // // Draw the Bezier curve
    // // Only if there are 4 points.. Or more.
    // // Like 4 + 3n points
    // // This to draw a continuous curve
    // // 4 for the first curve
    // // 3 for the next curve (last point of the previous curve is the first point of the next curve)
    //     // 1 last point
    //     // +2 control points
    //     // +1 last point (will be used as the first point of the next curve)
    // if model.points.len() >= 4 {
    //     let mut start_index = 0;
    //     let mut end_index = 3;

    //     while end_index < model.points.len() {
    //         // println!("Start index: {}, End index: {}", start_index, end_index);  // Debugging print statement
    //         let control_points = &model.points[start_index..=end_index].to_vec();
    //         bezier(&draw, &model, control_points);
    //         start_index += 3;
    //         end_index += 3;
    //     }
    // }  // 4 + 3n points
    // // Draw the lines
    // model.b_lines.iter().for_each(|(start, end)| {
    //     draw.line().start(*start).end(*end).weight(0.5).color(WHITE);
    // });

    // Draw the points
    // model.points.iter().for_each(|point| {draw.ellipse().xy(*point).radius(5.0).color(BLUEVIOLET);});

    let scale = 1.0;
    draw.polygon().points(
        model.control_points[0].iter().map(|(x, y)| pt2(x*scale, y*scale))
        ).color(WHITE);

    draw.to_frame(app, &frame).unwrap();  //* Draw all the elements above to the frame...
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

fn handle_key_event(
    app: &App,
    model: &mut Model, 
    key_code: Key
) {
    match key_code {
        Key::C => {
            model.points.clear();
            println!("Cleared all points");
        }
        Key::Escape => {
            println!("Exiting the application...");
            // std::process::exit(0);
        }
        // F to full screen
        Key::F => {
            let window = app.main_window();
            match window.is_fullscreen() {
                true => window.set_fullscreen(false),
                false => window.set_fullscreen(true),
            }
        }

        // * Move the control points
        Key::Up => model.control_points.iter_mut().for_each(|row| row.iter_mut().for_each(|(_, y)| *y += 10.0)),
        Key::Down => model.control_points.iter_mut().for_each(|row| row.iter_mut().for_each(|(_, y)| *y -= 10.0)),
        Key::Left => model.control_points.iter_mut().for_each(|row| row.iter_mut().for_each(|(x, _)| *x -= 10.0)),
        Key::Right => model.control_points.iter_mut().for_each(|row| row.iter_mut().for_each(|(x, _)| *x += 10.0)),

        // * Scale the control points
        // ^ Scale up by 10% (maintain aspect ratio) 
        Key::Equals => scale_control_points(&mut model.control_points, 1.1), // Scale up by 10%
        Key::Minus => scale_control_points(&mut model.control_points, 0.9), // Scale down by 10%
        // ^ Scale up by 10px (does not maintain aspect ratio)
        Key::W => scale_control_points_axis(&mut model.control_points, 1.1, 'y'), // Scale up Y axis
        Key::S => scale_control_points_axis(&mut model.control_points, 0.9, 'y'), // Scale down Y axis
        Key::A => scale_control_points_axis(&mut model.control_points, 0.9, 'x'), // Scale down X axis
        Key::D => scale_control_points_axis(&mut model.control_points, 1.1, 'x'), // Scale up X axis

        // * Rotate the control points
        Key::L => rotate_control_points(&mut model.control_points,  3.0),
        Key::R => rotate_control_points(&mut model.control_points, -3.0),

        _ => {println!("No action for key: {:?}", key_code);}
    }
}


fn rotate_control_points(control_points: &mut Vec<Vec<(f32, f32)>>, angle_degrees: f32) {
    // Calculate the center of the control points
    let mut center = (0.0, 0.0);
    let total_points = (control_points.len() * control_points[0].len()) as f32;
    for row in control_points.iter() {
        for &(x, y) in row.iter() {
            center.0 += x;
            center.1 += y;
        }
    }
    center.0 /= total_points;
    center.1 /= total_points;

    // Convert angle to radians
    let angle_radians = angle_degrees.to_radians();
    let cos_theta = angle_radians.cos();
    let sin_theta = angle_radians.sin();

    // Apply the rotation to each control point
    for row in control_points.iter_mut() {
        for (x, y) in row.iter_mut() {
            let translated_x = *x - center.0;
            let translated_y = *y - center.1;

            let rotated_x = translated_x * cos_theta - translated_y * sin_theta;
            let rotated_y = translated_x * sin_theta + translated_y * cos_theta;

            *x = rotated_x + center.0;
            *y = rotated_y + center.1;
        }
    }
}


fn scale_control_points(control_points: &mut Vec<Vec<(f32, f32)>>, scale_factor: f32) {
    // Calculate the center of the control points
    let mut center = (0.0, 0.0);
    let total_points = (control_points.len() * control_points[0].len()) as f32;
    for row in control_points.iter() {
        for &(x, y) in row.iter() {
            center.0 += x;
            center.1 += y;
        }
    }
    center.0 /= total_points;
    center.1 /= total_points;

    // Apply the scaling to each control point
    for row in control_points.iter_mut() {
        for (x, y) in row.iter_mut() {
            let translated_x = *x - center.0;
            let translated_y = *y - center.1;

            let scaled_x = translated_x * scale_factor;
            let scaled_y = translated_y * scale_factor;

            *x = scaled_x + center.0;
            *y = scaled_y + center.1;
        }
    }
}


fn scale_control_points_axis(control_points: &mut Vec<Vec<(f32, f32)>>, scale_factor: f32, axis: char) {
    // Calculate the center of the control points
    let mut center = (0.0, 0.0);
    let total_points = (control_points.len() * control_points[0].len()) as f32;
    for row in control_points.iter() {
        for &(x, y) in row.iter() {
            center.0 += x;
            center.1 += y;
        }
    }
    center.0 /= total_points;
    center.1 /= total_points;

    // Apply the scaling to each control point along the specified axis
    for row in control_points.iter_mut() {
        for (x, y) in row.iter_mut() {
            match axis {
                'x' => {
                    let translated_x = *x - center.0;
                    let scaled_x = translated_x * scale_factor;
                    *x = scaled_x + center.0;
                }
                'y' => {
                    let translated_y = *y - center.1;
                    let scaled_y = translated_y * scale_factor;
                    *y = scaled_y + center.1;
                }
                _ => {}
            }
        }
    }
}
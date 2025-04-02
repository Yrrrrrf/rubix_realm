#![allow(unused)]

// ? Library metadata -------------------------------------------------------------------------
use nannou::{
    draw,
    event::{ElementState, KeyboardInput},
    geom::point,
    image,
    prelude::*,
    winit::window::Icon,
};

pub mod n_test; // & nannou: library for creative coding in Rust
                // pub mod util;  // for now just contains `lorem ipsum` text generator
pub mod cube;
pub mod geometry;
pub mod math;

use math::bezier::*;
use math::surface::*;

// ? General library functionalities ----------------------------------------------------------

pub fn init_window() {
    nannou::app(model)
        // .update(update)  // Update the model here...<every frame>
        .event(event) // Handle events
        .simple_window(view) // Draw the model
        .run();
}

pub struct Model {
    // line_start_pos:Point2,  // The starting position of the line
    // b_lines: Vec<(Point2, Point2)>, // Each line is represented as a pair of start and end points
    points: Vec<Point3>,         // Stores the points added by mouse clicks
    control_points: Vec<Point3>, // Stores the control points for the Bezier curve
}

fn model(app: &App) -> Model {
    // * Set the window properties
    app.main_window()
        .set_min_inner_size_points(Some((720.0, 480.0)));
    app.main_window().set_inner_size_pixels(1920, 1080);
    // app.main_window().set_inner_size_pixels(1080, 720);
    app.main_window().set_title("Rubix Realm");
    app.main_window().set_window_icon(Some(
        load_icon("./assets/icons/rubik.png").expect("Failed to load window icon"),
    ));

    let scale = 300.0;

    // Define the cube's corner points
    let mut control_points = vec![
        Vec3::new(-1.0, -1.0, -1.0), // 0 (bottom, back, left)
        Vec3::new(-1.0, 1.0, -1.0),  // 1 (top, back, left)
        Vec3::new(1.0, 1.0, -1.0),   // 2 (top, back, right)
        Vec3::new(1.0, -1.0, -1.0),  // 3 (bottom, back, right)
        Vec3::new(-1.0, -1.0, 1.0),  // 4 (bottom, front, left)
        Vec3::new(-1.0, 1.0, 1.0),   // 5 (top, front, left)
        Vec3::new(1.0, 1.0, 1.0),    // 6 (top, front, right)
        Vec3::new(1.0, -1.0, 1.0),   // 7 (bottom, front, right)
    ];
    // now scale the control points
    control_points.iter_mut().for_each(|row| {
        *row *= scale;
    });

    // * Initialize the model
    Model {
        // line_start_pos: app.window_rect().xy(),
        // b_lines: Vec::new(),  // Initialize the lines vector
        points: Vec::new(), // Initialize the points vector
        // control_points: Vec::new(),  // Initialize the control points vector
        control_points,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _app.main_window()
        .set_title(&format!("{:.0} fps", _app.fps()));
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(MousePressed(MouseButton::Left)),
            ..
        } => {
            handle_mouse_event(app, model);
        }
        Event::WindowEvent {
            simple: Some(WindowEvent::KeyPressed(key_code)),
            ..
        } => {
            handle_key_event(app, model, key_code);
        }
        _ => {}
    }
}

/// View function that
// This fn is excecuted every frame
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK); // Clear the background to remove previous drawings

    // * Draw Edges
    let edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0), // back face
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4), // front face
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7), // connecting edges
    ];

    // add a line for each edge
    edges.iter().for_each(|(start, end)| {
        draw.line()
            .start(model.control_points[*start].into())
            .end(model.control_points[*end].into())
            .color(WHITE);
    });

    // * Draw Vertices
    model.control_points.iter().for_each(|point| {
        draw.ellipse()
            .color(BLUE)
            .w_h(10.0, 10.0)
            .x_y(point.x, point.y);
    });

    draw.to_frame(app, &frame).unwrap(); //* Draw all the elements above to the frame...
}

fn load_icon(icon_path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    let image_data = image::open(icon_path)?.to_rgba8();
    let (width, height) = image_data.dimensions();
    Ok(Icon::from_rgba(image_data.into_raw(), width, height).unwrap())
}

fn handle_mouse_event(app: &App, model: &mut Model) {
    let mouse_pos = app.mouse.position();
    let n_vec = Vec3::new(mouse_pos.x, mouse_pos.y, 0.0);
    println!("Mouse position: {:?}", mouse_pos); // Debugging print statement
    model.points.push(n_vec); // Add the point to the points vector
}

fn handle_key_event(app: &App, model: &mut Model, key_code: Key) {
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
            window.set_fullscreen(!window.is_fullscreen());
        }

        // * Move
        // move over x & y axis
        Key::Up => model
            .control_points
            .iter_mut()
            .for_each(|point| point.y += 10.0), // move up
        Key::Down => model
            .control_points
            .iter_mut()
            .for_each(|point| point.y -= 10.0), // move down
        Key::Left => model
            .control_points
            .iter_mut()
            .for_each(|point| point.x -= 10.0), // move left
        Key::Right => model
            .control_points
            .iter_mut()
            .for_each(|point| point.x += 10.0), // move right

        // * Rotate
        Key::U => rotate(&mut model.control_points, 10.0, 'x'), // Rotate up
        Key::D => rotate(&mut model.control_points, -10.0, 'x'), // Rotate down
        Key::L => rotate(&mut model.control_points, 10.0, 'y'), // Rotate right
        Key::R => rotate(&mut model.control_points, -10.0, 'y'), // Rotate left
        Key::I => rotate(&mut model.control_points, 10.0, 'z'), // Rotate in
        Key::O => rotate(&mut model.control_points, -10.0, 'z'), // Rotate out

        // * Scale
        // ^ Scale up by 10% (maintain aspect ratio)
        Key::Equals => scale(&mut model.control_points, 1.1), // Scale up by 10%
        Key::Minus => scale(&mut model.control_points, 0.9),  // Scale down by 10%
        // ^ Scale up by 10% (does not maintain aspect ratio)
        Key::X | Key::Y | Key::Z => {
            let axis = match key_code {
                Key::X => 'x',
                Key::Y => 'y',
                Key::Z => 'z',
                _ => unreachable!(), // This shouldn't happen
            };
            scale_axis(
                model.control_points.as_mut(),
                if app.keys.mods.shift() { 0.9 } else { 1.1 },
                axis,
            );
        }

        // * Reset
        Key::R => {
            model.control_points = vec![
                Vec3::new(-1.0, -1.0, -1.0), // 0 (bottom, back, left)
                Vec3::new(-1.0, 1.0, -1.0),  // 1 (top, back, left)
                Vec3::new(1.0, 1.0, -1.0),   // 2 (top, back, right)
                Vec3::new(1.0, -1.0, -1.0),  // 3 (bottom, back, right)
                Vec3::new(-1.0, -1.0, 1.0),  // 4 (bottom, front, left)
                Vec3::new(-1.0, 1.0, 1.0),   // 5 (top, front, left)
                Vec3::new(1.0, 1.0, 1.0),    // 6 (top, front, right)
                Vec3::new(1.0, -1.0, 1.0),   // 7 (bottom, front, right)
            ];
            model.control_points.iter_mut().for_each(|row| {
                *row *= 300.0;
            });
        }

        _ => {
            println!("No action for key: {:?}", key_code);
        }
    }
}

fn rotate(control_points: &mut Vec<Point3>, angle_degrees: f32, axis: char) {
    // Calculate the center of the control points
    let mut center = Point3::new(0.0, 0.0, 0.0);
    control_points.iter_mut().for_each(|point| center += *point);

    center /= control_points.len() as f32;

    // Convert angle to radians
    let angle_radians: f32 = angle_degrees.to_radians();
    let cos_theta: f32 = angle_radians.cos();
    let sin_theta: f32 = angle_radians.sin();

    // Apply the rotation to each control point
    for point in control_points.iter_mut() {
        let translated_point = *point - center;
        let rotated_point = match axis {
            'x' => Point3::new(
                translated_point.x,
                translated_point.y * cos_theta - translated_point.z * sin_theta,
                translated_point.y * sin_theta + translated_point.z * cos_theta,
            ),
            'y' => Point3::new(
                translated_point.x * cos_theta + translated_point.z * sin_theta,
                translated_point.y,
                -translated_point.x * sin_theta + translated_point.z * cos_theta,
            ),
            'z' => Point3::new(
                translated_point.x * cos_theta - translated_point.y * sin_theta,
                translated_point.x * sin_theta + translated_point.y * cos_theta,
                translated_point.z,
            ),
            _ => Point3::new(0.0, 0.0, 0.0),
        };

        *point = rotated_point + center;
    }
}

fn scale(control_points: &mut Vec<Point3>, scale_factor: f32) {
    control_points
        .iter_mut()
        .for_each(|point| *point *= scale_factor);
}

fn scale_axis(control_points: &mut Vec<Point3>, scale_factor: f32, axis: char) {
    control_points.iter_mut().for_each(|point| match axis {
        'x' => point.x *= scale_factor,
        'y' => point.y *= scale_factor,
        'z' => point.z *= scale_factor,
        _ => (),
    });
}

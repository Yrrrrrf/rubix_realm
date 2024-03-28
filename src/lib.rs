#![allow(unused)]


// ? Library metadata -------------------------------------------------------------------------
use nannou::{image, prelude::*, winit::window::Icon};

pub mod n_test;  // & nannou: library for creative coding in Rust
// pub mod util;  // for now just contains `lorem ipsum` text generator
pub mod math;
pub mod geometry;

use math::bezier::*;


// ? General library functionalities ----------------------------------------------------------


pub fn init_window() {
    nannou::app(model)
        .update(update)  // Update the model here...<every frame>
        .event(event)  // Handle events
        .simple_window(view)  // Draw the model
        .run();
}




pub struct Model {
    // This will now represent the start point for the next line
    line_start_pos: Point2,
    b_lines: Vec<(Point2, Point2)>, // Each line is represented as a pair of start and end points
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
        line_start_pos: app.window_rect().xy(),
        b_lines: Vec::new(), // Initialize the lines vector
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // _app.main_window().set_title(&format!("Drawing Lines - {:.0} fps", _app.fps()));
    // do something similar as above, but this time just add the fps to the title instead of the whole string
    _app.main_window().set_title(&format!("{:.0} fps", _app.fps()));
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple: Some(MousePressed(_)), .. } => {
            let mouse_pos = app.mouse.position();
            model.b_lines.push((model.line_start_pos, mouse_pos));
            // println!("{:?}", model.b_lines);
            model.line_start_pos = mouse_pos;
        },
        // Event::WindowEvent { simple: Some(MouseMoved(_)), .. } => {
        //     println!("Mouse moved to: {:?}", app.mouse.position());
        // },
        _ => {}
    }
}

/// View function that 
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK); // Clear the background to remove previous drawings

    // Draw all lines that have been added to the model.lines vector.
    // model.b_lines.iter().for_each(|line| {draw.line().start(line.0).end(line.1).weight(2.0).rgb(1.0, 0.0, 0.0);});
    // Draw the current (dynamic) line that follows the mouse
    // draw.line().start(model.line_start_pos).end(app.mouse.position()).weight(2.0).rgb(0.5, 0.5, 0.5);

    bezier(&app, &model, &frame);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();  //* Draw all the elements above to the frame...
}


fn load_icon(icon_path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    let image_data = image::open(icon_path)?.to_rgba8();
    let (width, height) = image_data.dimensions();
    Ok(Icon::from_rgba(image_data.into_raw(), width, height).unwrap())
}

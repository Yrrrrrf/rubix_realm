// use nannou::{
//     prelude::*,
//     color,
// };

// use crate::{
//     geometry::n_cube::Hypercube,
//     util::lorem::lorem
// };

// pub fn nannou_test() {
//     // nannou::sketch(view).run();
//     nannou::app(model)
//         // .update(update)
//         .run();

// }

// struct Model {
//     // Store the window ID so we can refer to our window later
//     // _window: window::Id,
//     last_point: Point2,
// }

// fn model(app: &App) -> Model {
//     let _window = app.new_window()
//         .size(1080, 720)
//         .sketch(view)
//         .build()
//         .unwrap()
//         ;

//         Model {
//             // _window,
//             last_point: pt2(0.0, 0.0)
//         }
// }

// fn update(_app: &App, _model: &mut Model, _update: Update) {
//     // Update the model here...
//     // print the actual number of frames per second
//     // println!("FPS: {}", _app.fps());
// }

// fn view(app: &App, frame: Frame) {
//     // main drawing
//     let draw = app.draw();  // Create a new draw object (mainly used for drawing shapes and text to the screen)
//     // draw.background().color(DARKSLATEGRAY);  // Set the background color to black
//     // aux
//     let r = app.window_rect();
//     let t = app.time;

//     // * Drawings
//     // vec_field(app, &draw);
//     // draw_square(app, &draw, r);
//     // n_size_polygon(app, &draw);
//     // some_example(app, frame);

//     // create a loop of 2 lines that follow the mouse
//     // let start = pt2(0.0, 0.0);
//     // draw.line().start(start).end(pt2(app.mouse.x, app.mouse.y)).color(RED);
//     // draw.line().start(start).end(pt2(-app.mouse.x, -app.mouse.y)).color(WHITE);

//     // draw_vec_space_gizmo(app, &draw);
//     // draw_origin(app, &draw);

//     // test_camera(app, &draw);

//     draw_lines(app, &draw);

//     // * Write the result of the drawing to the window's frame
//     draw.to_frame(app, &frame).unwrap();
// }

// fn draw_lines(app: &App, draw: &Draw, model: &mut Model) {
//     let r = app.window_rect();
//     let t = app.time;

//     // Draw a point at the last_point
//     draw.ellipse()
//         .x_y(model.last_point.x, model.last_point.y)
//         .w_h(5.0, 5.0)
//         .color(WHITE);

//     // Check if the mouse is within the window bounds before drawing
//     if r.contains(app.mouse.position()) {
//         // Draw a line from the last_point to the current mouse position
//         draw.line()
//             .start(model.last_point)
//             .end(pt2(app.mouse.x, app.mouse.y))
//             .color(RED);

//         // Update last_point to be the current mouse position
//         model.last_point = pt2(app.mouse.x, app.mouse.y);
//     }
// }

// // * TESTING ELEMENTS (?)
// fn draw_origin(app: &App, draw: &Draw) {
//     let r = app.window_rect();
//     draw.line().start(pt2(r.left(), 0.0)).end(pt2(r.right(), 0.0)).color(RED).weight(3.0);
//     draw.line().start(pt2(0.0, r.bottom())).end(pt2(0.0, r.top())).color(GREEN).weight(3.0);
// }

// fn draw_vec_space_gizmo(app: &App, draw: &Draw) {
//     let r = app.window_rect();
//     // let t = app.time;

//     // Y axis
//     (r.bottom() as i32..=r.top() as i32).for_each(|i| if i % 100 == 0 {
//         draw.line().start(pt2(r.left(), i as f32)).end(pt2(r.right(), i as f32)).color(color::rgb8(128, 0, 0));
//     });
//     // X axis
//     (r.left() as i32..=r.right() as i32).for_each(|i| if i % 100 == 0 {
//         draw.line().start(pt2(i as f32, r.bottom())).end(pt2(i as f32, r.top())).color(color::rgb8(0, 128, 0));
//     });

// }

// fn draw_square(app: &App, draw: &Draw, r: geom::Rect) {
//     // let square = geom::Rect::from_w_h(100.0, 100.0).top_left_of(r);
//     // draw.rect().xy(square.xy()).wh(square.wh()).color(RED);

//     // draw anotehr in the middle
//     // let square = geom::Rect::from_w_h(100.0, 100.0).middle_of(r);
//     // draw.rect().xy(square.xy()).wh(square.wh()).color(GREEN);

//     // let hypercube_3d = Hypercube::new(3, 1.0);
//     // println!("3D Hypercube vertices: {:?}", hypercube_3d.vertices);

//     let vertices = [
//         (0.0, 0.0, 0.0), // 000
//         (0.0, 0.0, 1.0), // 001
//         (0.0, 1.0, 0.0), // 010
//         (0.0, 1.0, 1.0), // 011
//         (1.0, 0.0, 0.0), // 100
//         (1.0, 0.0, 1.0), // 101
//         (1.0, 1.0, 0.0), // 110
//         (1.0, 1.0, 1.0), // 111
//     ];

//     // MODIFY IT TO BE 100X BIGGER
//     let vertices: Vec<(f64, f64, f64)> = vertices.iter().map(|(x, y, z)| (x * 100.0, y * 100.0, z * 100.0)).collect();

//     let edges = [
//         // 000 to 001, 010, 100
//         (0, 1), (0, 2), (0, 4), (1, 3), (1, 5), (2, 3),
//         (2, 6), (3, 7), (4, 5), (4, 6), (5, 7), (6, 7),
//     ];

//     for (start, end) in edges.iter() {
//         draw.line()
//             .start(pt2(vertices[*start].0 as f32, vertices[*start].1 as f32))
//             .end(pt2(vertices[*end].0 as f32, vertices[*end].1 as f32))
//             .color(WHITE);
//     }
// }

// fn test_camera(app: &App, draw: &Draw) {
//     // Set up the camera view matrix (e.g., position and direction)
//     let eye = pt3(0.0, 0.0, 1.0); // Camera position in space
//     let target = pt3(0.0, 0.0, 0.0); // Point the camera is looking at
//     let up = vec3(0.0, 1.0, 0.0); // The "up" direction for the camera (usually y-axis)
//     let view = Mat4::look_at_rh(eye, target, up);

//     // Set up the projection matrix (e.g., perspective or orthographic)
//     let aspect_ratio = app.window_rect().w() / app.window_rect().h();
//     let fov = std::f32::consts::PI / 4.0; // Field of view (radians)
//     let near = 0.01; // Near clipping plane
//     let far = 1000.0; // Far clipping plane
//     let proj = Mat4::perspective_rh(fov, aspect_ratio, near, far);
// }

// // * PASSED (✓)
// fn vec_field(app: &App, draw: &Draw) {
//     let r = app.window_rect();
//     for r in r.subdivisions_iter() {
//         for r in r.subdivisions_iter() {
//             for r in r.subdivisions_iter() {
//                 let side = r.w().min(r.h());
//                 let start = r.xy();
//                 let start_to_mouse = app.mouse.position() - start;
//                 let target_mag = start_to_mouse.length().min(side * 0.5);
//                 let end = start + start_to_mouse.normalize_or_zero() * target_mag;
//                 draw.arrow().weight(5.0).points(start, end);
//             }
//         }
//     }
// }

// fn n_size_polygon(app: &App, draw: &Draw) {
//     let win = app.window_rect();
//     let t = app.time;

//     // Create an `ngon` of points.
//     let n_points = 5;
//     let radius = win.w().min(win.h()) * 0.25;
//     let points = (0..n_points).map(|i| {
//         let fract = i as f32 / n_points as f32;
//         let phase = fract;
//         let x = radius * (TAU * phase).cos();
//         let y = radius * (TAU * phase).sin();
//         pt2(x, y)
//     });
//     draw.polygon()
//         .x(-win.w() * 0.25)
//         .color(WHITE)
//         .rotate(-t * 0.1)
//         .stroke(PINK)
//         .stroke_weight(20.0)
//         .join_round()
//         .points(points);

//     // Do the same, but give each point a unique colour.
//     let n_points = 7;
//     let points_colored = (0..n_points).map(|i| {
//         let fract = i as f32 / n_points as f32;
//         let phase = fract;
//         let x = radius * (TAU * phase).cos();
//         let y = radius * (TAU * phase).sin();
//         let r = fract;
//         let g = 1.0 - fract;
//         let b = (0.5 + fract) % 1.0;
//         (pt2(x, y), rgb(r, g, b))
//     });
//     draw.polygon()
//         .x(win.w() * 0.25)
//         .rotate(t * 0.2)
//         .points_colored(points_colored);
// }

// fn some_example(app: &App, frame: Frame) {
//     // Begin drawing
//     let draw = app.draw();

//     // Clear the background to blue.
//     draw.background().color(CORNFLOWERBLUE);

//     // Draw a purple triangle in the top left half of the window.
//     let win = app.window_rect();
//     draw.tri()
//         .points(win.bottom_left(), win.top_left(), win.top_right())
//         .color(VIOLET);

//     // Draw an ellipse to follow the mouse.
//     let t = app.time;
//     draw.ellipse()
//         .x_y(app.mouse.x * t.cos(), app.mouse.y)
//         .radius(win.w() * 0.125 * t.sin())
//         .color(RED);

//     // Draw a line!
//     draw.line()
//         .weight(10.0 + (t.sin() * 0.5 + 0.5) * 90.0)
//         .caps_round()
//         .color(PALEGOLDENROD)
//         .points(win.top_left() * t.sin(), win.bottom_right() * t.cos());

//     // Draw a quad that follows the inverse of the ellipse.
//     draw.quad()
//         .x_y(-app.mouse.x, app.mouse.y)
//         .color(DARKGREEN)
//         .rotate(t);

//     // Draw a rect that follows a different inverse of the ellipse.
//     draw.rect()
//         .x_y(app.mouse.y, app.mouse.x)
//         .w(app.mouse.x * 0.25)
//         .hsv(t, 1.0, 1.0);

//     // Write the result of our drawing to the window's frame.
//     draw.to_frame(app, &frame).unwrap();
// }

#![allow(unused)]


// ? Library metadata -------------------------------------------------------------------------
// * External crates used in the library
// #[cfg(feature = "3d_render")]
// extern crate winit;
// #[cfg(feature = "3d_render")]
// extern crate image;

// * Modules used in the library
pub mod render;  // & (in case to use winit, image and wgpu)
pub mod math;


// ? General library functionalities ----------------------------------------------------------
// pub fn general_functionality() {
//     // Code for general functionality of your library
// }

// #[cfg(feature = "3d_render")]
// pub fn render_specific_functionality() {
//     // Code specific to 3D rendering feature
// }




// use std::path::Path;

// use render::{
//     input::*,
//     win_state::State,
// };

// use winit::{
//     dpi::LogicalSize,
//     event_loop::EventLoop,
//     keyboard::ModifiersState,
//     event::{
//         Event, 
//         WindowEvent,
//     },
//     window::{
//         Icon, 
//         WindowBuilder
//     }
// };

// use wgpu::SurfaceError::*;


// // State::new uses async code, so we're going to wait for it to finish
// pub async fn init_window() -> Result<(), impl std::error::Error> {
//     env_logger::init();
 
//     let event_loop = EventLoop::new()?;

//     // ^ Move the path to a separate file like "Constants.rs" or "Config.toml" or something like that
//     let icon = load_icon(Path::new("./assets/icons/rubik.png"));
//     let mut modifiers = ModifiersState::default();
   
//     let window = WindowBuilder::new()
//         .with_title("An iconic window!")
//         .with_window_icon(Some(icon))
//         .with_inner_size(LogicalSize::new(1080, 720))
//         .with_min_inner_size(LogicalSize::new( 720,  480))
//         .with_max_inner_size(LogicalSize::new(1920, 1080))
//         .build(&event_loop)?;

//     // let mut state = State::new(&window).await;

//     event_loop.run(move |event, elwt| {
//         if let Event::WindowEvent { event, .. } = event {
//             match event {
//                 // * Interacting with the window
//                 WindowEvent::CloseRequested => elwt.exit(),
//                 WindowEvent::KeyboardInput { event, .. } => handle_keyboard_input(&event, &mut modifiers, &elwt),
//                 WindowEvent::MouseInput { state, button, .. } => handle_mouse_input(&state, &button, &elwt),
//                 // WindowEvent::MouseWheel { delta, .. } => handle_mouse_wheel(&delta, &elwt),  
//                 WindowEvent::CursorMoved { position, .. } => {
//                     handle_cursor_moved(&position, &elwt);
//                     // state.input(&event);
//                 },

//                 // // * Modifiers (shift, ctrl, alt, etc.)
//                 // WindowEvent::ModifiersChanged(new) => modifiers = new.state(),

//                 // // * Updating the window (resizing, redrawing, etc.)
//                 // WindowEvent::Resized(physical_size) => state.resize(physical_size),
//                 // WindowEvent::RedrawRequested => {
//                 //     state.window().request_redraw();
//                 //     state.update();
//                 //     if let Err(surface_error) = state.render() {  // Do something with the error, else it will be ignored
//                 //         match surface_error {
//                 //             Lost | Outdated => state.resize(state.size),
//                 //             OutOfMemory => {log::error!("OutOfMemory"); elwt.exit();}
//                 //             Timeout => log::warn!("Surface timeout"),
//                 //         }
//                 //     }
//                 // },
//                 _ => (),
//             }

//         }
//     })
// }


// fn load_icon(path: &Path) -> Icon {
//     let (icon_rgba, icon_width, icon_height) = {
//         let image = image::open(path)
//             .expect("Failed to open icon path")
//             .into_rgba8();
//         let (width, height) = image.dimensions();
//         let rgba = image.into_raw();
//         (rgba, width, height)
//     };
//     Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
// }


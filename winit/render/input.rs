#![allow(unused_variables)]

use winit::{
    event_loop::EventLoopWindowTarget, 
    event::{
        ElementState, 
        KeyEvent,
        MouseButton,
        MouseScrollDelta,
    }, 
    keyboard::{
        Key, 
        ModifiersState, 
        NamedKey
    },
};


pub fn handle_keyboard_input(
    event: &KeyEvent, 
    modifiers: &mut ModifiersState, 
    elwt: &EventLoopWindowTarget<()>
) {
    if event.logical_key == Key::Named(NamedKey::Escape) {
        elwt.exit();
    }

    if event.state == ElementState::Pressed && !event.repeat {
        let modifier_str = [
            (modifiers.shift_key(), "Shift"),
            (modifiers.control_key(), "Ctrl"),
            (modifiers.alt_key(), "Alt"),
            (modifiers.super_key(), "Super"),
        ].iter().filter(|(key, _)| *key).map(|(_, name)| *name)
        .collect::<Vec<&str>>()  // Collect the modifier names into a vector.
        .join(" + ");  // Add a space between each modifier.

        match modifier_str.is_empty() {
            true => println!("{:?}", event.logical_key),
            false => println!("{} + {:?}", modifier_str, event.logical_key),
        }

    }
}


pub fn handle_mouse_input(
    state: &ElementState, 
    button: &MouseButton,
    elwt: &EventLoopWindowTarget<()>
) {
    if *state == ElementState::Pressed {
        println!("{button:10?}");
    }
}


pub fn handle_mouse_wheel(
    delta: &MouseScrollDelta,
    elwt: &EventLoopWindowTarget<()>
) {
    println!("{delta:?}");
}


pub fn handle_cursor_moved(
    position: &winit::dpi::PhysicalPosition<f64>,
    elwt: &EventLoopWindowTarget<()>
) {
    // println!("Cursor moved to: {:?}", position);
    // Add your logic for cursor movement here
}
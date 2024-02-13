use std::path::Path;

use winit::{
    event::{
        ElementState, 
        Event, 
        KeyEvent, 
        WindowEvent
    }, 
    event_loop::{
        EventLoop, 
        EventLoopWindowTarget
    }, 
    keyboard::{
        Key, 
        ModifiersState, 
        NamedKey
    },
    window::{
        Icon, 
        WindowBuilder
    }
};


pub fn init_window() -> Result<(), impl std::error::Error> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "./assets/icons/rubik.png");
    let icon = load_icon(Path::new(path));

    let mut modifiers = ModifiersState::default();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("An iconic window!")
        .with_window_icon(Some(icon))
        .with_inner_size(winit::dpi::LogicalSize::new(1080, 720))
        .with_min_inner_size(winit::dpi::LogicalSize::new( 720,  480))
        .with_max_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, elwt| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::KeyboardInput { event, .. } => handle_keyboard_input(&event, &mut modifiers, &elwt),
                WindowEvent::ModifiersChanged(new) => modifiers = new.state(),
                WindowEvent::RedrawRequested => window.request_redraw(),
                _ => (),
             }
        }
    })

}


fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}


fn handle_keyboard_input(
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

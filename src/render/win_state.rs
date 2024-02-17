use std::iter;

use winit::{
    event::*,
    window::Window,
    dpi::PhysicalSize,
};

use super::{
    pipeline, 
    surface
};

pub struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    // The window must be declared after the surface so it gets dropped after it as the surface contains unsafe references to the window's resources.
    window: &'a Window,
    clear_color: wgpu::Color,

    render_pipeline: wgpu::RenderPipeline,
}

impl<'a> State<'a> {
    pub async fn new(window: &'a Window) -> State<'a> {
        let (surface, device, queue, config, size) = surface::create_surface(window).await;
        // Load the shader module from the Wgsl source
        let shader_module = device.create_shader_module(
            wgpu::include_wgsl!("../../assets/shaders/triangle.wgsl")
            // wgpu::include_wgsl!("../assets/shaders/shader.wgsl")
        );
        let render_pipeline = pipeline::set_pipeline(&device, &config, shader_module);
        let clear_color = wgpu::Color::BLACK;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            clear_color,
            render_pipeline,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.clear_color = wgpu::Color {
                    r: position.x as f64 / self.size.width as f64,
                    g: position.y as f64 / self.size.height as f64,
                    b: 1.0,
                    a: 1.0,
                };
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        
            render_pass.set_pipeline(&self.render_pipeline); // 2.
            render_pass.draw(0..3, 0..1); // 3.
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

// event_loop.run(move |event, elwt| {
//     if let Event::WindowEvent { event, .. } = event {
//         match event {
//             // * Interacting with the window
//             WindowEvent::CloseRequested => elwt.exit(),
//             WindowEvent::KeyboardInput { event, .. } => handle_input(&event, &mut modifiers, &elwt),
//             WindowEvent::ModifiersChanged(new) => modifiers = new.state(),
//             // * Updating the window (resizing, redrawing, etc.)
//             WindowEvent::Resized(physical_size) => state.resize(physical_size),
//             WindowEvent::RedrawRequested => {
//                 // This tells winit that we want another frame after this one
//                 state.window().request_redraw();
//                 state.update();
//                 match state.render() {
//                     Ok(_) => {}
//                     // Reconfigure the surface if it's lost or outdated
//                     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
//                         state.resize(state.size)
//                     }
//                     // The system is out of memory, we should probably quit
//                     Err(wgpu::SurfaceError::OutOfMemory) => {
//                         log::error!("OutOfMemory");
//                         elwt.exit();
//                     }

//                     // This happens when the a frame takes too long to present
//                     Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
//                 }
//             }
//             _ => (),
//         }

//     }
// })
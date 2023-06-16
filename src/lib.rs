pub mod state;
pub use state::*;

pub mod window;
pub use window::*;

pub mod primitives;
pub use primitives::*;

pub mod render;
pub use render::*;

pub mod instance;
pub use instance::*;

pub mod camera;
pub use camera::*;

pub async fn init() {
    env_logger::init();

    let (event_loop, window) = new_window();
    let mut state = HardwareState::new(window).await;

    let shader = Shader::new(&state, "res/shader.wgsl", "vert", "frag", Some("shader module"));
    let mut camera = Camera::new(&state, glam::vec3(0.0, 0.0, -10.0), glam::Quat::IDENTITY, 45.0, 0.1, 100.0);
    camera.resize(&state);
    let renderer = Renderer::new(&state, &[camera.bind_group_layout()], vec![camera.create_bind_group(&state)], vec![], &shader);

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent 
            { 
                window_id,
                event, 
                .. 
            } => if event == winit::event::WindowEvent::CloseRequested && window_id == state.window().id() {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            },
            winit::event::Event::MainEventsCleared => {
                update(&state);
                state.window().request_redraw();
            }
            winit::event::Event::RedrawRequested(_) => {
                match renderer.render(&state) {
                    Ok(_) => (),
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize();
                        camera.resize(&state);
                    },
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = winit::event_loop::ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            _ => (),
        }
    });
}

pub fn update(_state: &HardwareState) {

}


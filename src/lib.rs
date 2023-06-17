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

pub mod texture;
pub use texture::*;

pub async fn init() {
    env_logger::init();

    let (event_loop, window) = new_window();
    let mut state = HardwareState::new(window).await;

    let shader = Shader::new(&state, "res/shader.wgsl", "vert", "frag", Some("shader module"));

    let mouse_sensitivity = 0.3;
    let player_speed = 10.0;
    let mut camera = Camera::new(&state, glam::vec3(0.0, 0.0, 10.0), 45.0, 0.1, 10000.0, mouse_sensitivity, player_speed);
    camera.resize(&state);

    let mut renderer = Renderer::new(&state, &[camera.bind_group_layout()], vec![camera.create_bind_group(&state)], vec![], &shader);
    let start_time = std::time::Instant::now();
    let mut last_frame_time = start_time;

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent 
            { 
                window_id,
                event, 
                .. 
            } => if window_id == state.window().id() {
                if !camera.handle_keyboard_input(&event) {
                    match event {
                        winit::event::WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                        winit::event::WindowEvent::Resized(size) => {
                            resize(&mut state, &mut camera, &mut renderer, size);
                        },
                        winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            resize(&mut state, &mut camera, &mut renderer, *new_inner_size);
                        },
                        _ => (),
                    }
                }
            },
            winit::event::Event::MainEventsCleared => {
                update(&state, &start_time, &last_frame_time);
                camera.update(&state, last_frame_time.elapsed().as_secs_f32());
                state.window().request_redraw();
                last_frame_time = std::time::Instant::now();
            }
            winit::event::Event::RedrawRequested(_) => {
                match renderer.render(&state) {
                    Ok(_) => (),
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window().inner_size();
                        resize(&mut state, &mut camera, &mut renderer, size); 
                    },
                    // The system is out of memory
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = winit::event_loop::ControlFlow::Exit,
                    // Ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            },
            winit::event::Event::DeviceEvent { event, .. } => {
                camera.handle_mouse_input(&event);
            },
            _ => (),
        }
    });
}

pub fn resize(state: &mut HardwareState, camera: &mut Camera, renderer: &mut Renderer, size: winit::dpi::PhysicalSize<u32>) {
    state.resize(size);
    camera.resize(state);
    renderer.resize(state, size);
}

pub fn update(
    _state: &HardwareState, 
    _start_time: &std::time::Instant, 
    _last_frame_time: &std::time::Instant
) {

}


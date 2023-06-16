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

pub async fn init() {
    env_logger::init();

    let (event_loop, window) = new_window();
    let state = HardwareState::new(window).await;

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
                update(&event, &state);
                state.window().request_redraw();
            }
            winit::event::Event::RedrawRequested(_) => {
                render(&state);
            }
            _ => (),
        }
    });
}

pub fn update(event: &winit::window::Event, state: &HardwareState) {

}


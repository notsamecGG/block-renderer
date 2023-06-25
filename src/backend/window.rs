pub fn new_window() -> (winit::event_loop::EventLoop<()>, winit::window::Window) {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Renderer")
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    match window.set_cursor_grab(winit::window::CursorGrabMode::Locked) {
        Ok(_) => (),
        Err(_) => log::warn!("Unable to grab cursor"),
    };

    (event_loop, window)
}

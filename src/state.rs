pub struct HardwareState {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_format: wgpu::TextureFormat,
    window: winit::window::Window,
}

impl HardwareState {
    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn surface_format(&self) -> &wgpu::TextureFormat {
        &self.surface_format
    }
}

impl HardwareState {
    pub async fn new(window: winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { 
            backends: wgpu::Backends::PRIMARY, 
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_format = configure_surface(&device, &surface, &adapter, window.inner_size());

        Self {
            adapter,
            device,
            queue,
            surface,
            surface_format,
            window,
        }
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.surface_format = configure_surface(&self.device, &self.surface, &self.adapter, size);
    }

}


fn configure_surface(device: &wgpu::Device, surface: &wgpu::Surface, adapter: &wgpu::Adapter, window_size: winit::dpi::PhysicalSize<u32>) -> wgpu::TextureFormat {
    let caps = surface.get_capabilities(adapter);
    let format = caps.formats
        .iter()
        .find(|format| format.is_srgb())
        .unwrap_or(&caps.formats[0]);

    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: *format,
        width: window_size.width,
        height: window_size.height,
        present_mode: caps.present_modes[0],
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &surface_config);
    format.clone()
}

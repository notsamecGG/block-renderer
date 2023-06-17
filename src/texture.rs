use crate::HardwareState;


pub fn create_texture(state: &HardwareState, size: winit::dpi::PhysicalSize<u32>, label: Option<&str>) -> (wgpu::Texture, wgpu::TextureView) {
    let size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };

    let desc = wgpu::TextureDescriptor {
        label,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: Texture::DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[Texture::DEPTH_FORMAT],
    }; 

    let texture = state.device().create_texture(&desc);
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    (texture, view)
}



pub struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
    
    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;


    pub fn create_depth_texture(state: &HardwareState) -> Self {
        let (texture, view) = create_texture(state, state.window().inner_size(), Some("Depth Texture"));

        let sampler = state.device().create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Depth Texture Sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        
        Self { 
            texture, 
            view, 
            sampler
        }
    }

    pub fn resize_texture(&mut self, state: &HardwareState, size: winit::dpi::PhysicalSize<u32>) {
        self.texture.destroy();
        
        let (texture, view) = create_texture(state, size, Some("Depth Texture"));

        self.texture = texture; 
        self.view = view;
    }
}

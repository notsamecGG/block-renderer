use crate::HardwareState;


pub fn create_texture(
    state: &HardwareState,
    size: winit::dpi::PhysicalSize<u32>,
    format: wgpu::TextureFormat,
    sample_count: u32,
    label: Option<&str>
) -> (wgpu::Texture, wgpu::TextureView) {

    let size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };

    let desc = wgpu::TextureDescriptor {
        label,
        size,
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[format],
    }; 

    let texture = state.device().create_texture(&desc);
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    (texture, view)
}



pub struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,

    format: wgpu::TextureFormat,
    sample_count: u32,
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


    pub fn create_depth_texture(state: &HardwareState, sample_count: u32) -> Self {
        let (texture, view) = create_texture(state, state.window().inner_size(), Self::DEPTH_FORMAT, sample_count, Some("Depth Texture"));

        let sampler = state.device().create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Depth Texture Sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        
        // let (bind_group_layout, bind_group) = Self::create_bind_group(state, &view, &sampler, wgpu::TextureSampleType::Depth);
        
        Self { 
            texture, 
            view, 
            sampler,
            format: Self::DEPTH_FORMAT,
            sample_count
        }
    }

    pub fn create_texture(state: &HardwareState, size: winit::dpi::PhysicalSize<u32>, format: wgpu::TextureFormat, sample_count: u32, label: Option<&str>) -> Self {
        let (texture, view) = create_texture(state, size, format, sample_count, label);

        let sampler = state.device().create_sampler(&wgpu::SamplerDescriptor { 
            label: Some("Texture Sampler"), 
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self { 
            texture, 
            view, 
            sampler,
            format,
            sample_count
        }
    }

    pub fn resize_texture(&mut self, state: &HardwareState, size: winit::dpi::PhysicalSize<u32>) {
        self.texture.destroy();
        
        let (texture, view) = create_texture(state, size, self.format, self.sample_count, Some("Depth Texture"));

        self.texture = texture; 
        self.view = view;
    }
}

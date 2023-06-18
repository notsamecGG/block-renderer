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

    bind_group: Option<wgpu::BindGroup>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
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

    pub fn bind_group(&self) -> Option<&wgpu::BindGroup> {
        self.bind_group.as_ref()
    }

    pub fn bind_group_layout(&self) -> Option<&wgpu::BindGroupLayout> {
        self.bind_group_layout.as_ref()
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
        
        let (bind_group_layout, bind_group) = Self::create_bind_group(state, &view, &sampler, wgpu::TextureSampleType::Depth);
        
        Self { 
            texture, 
            view, 
            sampler,
            bind_group: Some(bind_group),
            bind_group_layout: Some(bind_group_layout),
        }
    }

    pub fn resize_texture(&mut self, state: &HardwareState, size: winit::dpi::PhysicalSize<u32>) {
        self.texture.destroy();
        
        let (texture, view) = create_texture(state, size, Some("Depth Texture"));

        self.texture = texture; 
        self.view = view;
    }

    // pub fn create_bind_group(state: &HardwareState, view: &wgpu::TextureView, sampler: &wgpu::Sampler, sample_type: wgpu::TextureSampleType) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    //     let layout = state.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
    //         label: Some("Texture Bind Group Layout"), 
    //         entries: &[wgpu::BindGroupLayoutEntry {
    //             binding: 0,
    //             visibility: wgpu::ShaderStages::FRAGMENT,
    //             ty: wgpu::BindingType::Texture {
    //                 multisampled: false,
    //                 view_dimension: wgpu::TextureViewDimension::D2,
    //                 sample_type,
    //             },
    //             count: None,
    //         }, wgpu::BindGroupLayoutEntry {
    //             binding: 1,
    //             visibility: wgpu::ShaderStages::FRAGMENT,
    //             ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
    //             count: None,
    //         }],
    //     });
    //
    //     let bind_group = state.device().create_bind_group(&wgpu::BindGroupDescriptor {
    //         label: Some("Texture Bind Group"),
    //         layout: &layout,
    //         entries: &[wgpu::BindGroupEntry {
    //             binding: 0,
    //             resource: wgpu::BindingResource::TextureView(view),
    //         }, wgpu::BindGroupEntry {
    //             binding: 1,
    //             resource: wgpu::BindingResource::Sampler(sampler),
    //         }],
    //     });
    //
    //     (layout, bind_group)
    // }
}

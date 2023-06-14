use crate::Shader;

pub fn new_render_pipeline(
    state: &HardwareState, 
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    shader: &Shader
) -> (wgpu::RenderPipeline, wgpu::PipelineLayout) {

    let layout = state.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    let render_pipeline = state.device().create_render_pipeline(wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState { 
            module: shader.module(),
            entry_point: shader.vertex_entry(), 
            buffers: shader.vertex_buffers_layout(),
        },
        fragment: Some(wgpu::FragmentState { 
            module: shader.module(), 
            entry_point: shader.fragment_entry(), 
            targets: &[Some(wgpu::ColorTargetState { 
                format: state.surface_format(),
                blend: Some(wgpu::BlendState::REPLACE), 
                write_mask: wgpu::ColorWrite::ALL 
            })],
        }),
        primitive: wgpu::PrimitiveState { 
            topology: wgpu::PrimitiveTopology::TriangleList, 
            strip_index_format: None, 
            front_face: wgpu::FrontFace::Ccw, 
            cull_mode: Some(wgpu::Face::Back), 
            unclipped_depth: false, 
            polygon_mode: wgpu::PolygonMode::Fill, 
            conservative: false 
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState { 
            count: 1, 
            mask: !0, 
            alpha_to_coverage_enabled: false
        },
        multiview: None,
    });

    (render_pipeline, layout)
}

pub fn render(state: &HardwareState) -> Result<(), wgpu::SurfaceError> {
    let mut encoder = state.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });
    
    let texture = state.surface().get_current_texture()?;
    let view = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
            label: Some("Render Pass"), 
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                view: &view, 
                resolve_target: None, 
                ops: wgpu::Operations { 
                    load: wgpu::LoadOp::Clear(
                        wgpu::Color { 
                            r: 0.2, 
                            g: 0.2, 
                            b: 0.1, 
                            a: 1.0 
                        }
                    ), 
                    store: true
                }
            })], 
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline();
    }

    state.queue().submit(std::iter::once(encoder.finish()));
    texture.present();

    Ok(())
}

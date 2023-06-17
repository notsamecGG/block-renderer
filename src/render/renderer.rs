use wgpu::util::DeviceExt;

use crate::{HardwareState, Shader, RenderSet, Vertex, Descriptable, QUAD_INDICES, QUAD_VERTICES, Texture};


pub struct Renderer {
    render_pipeline: wgpu::RenderPipeline,

    vertices_buffer: wgpu::Buffer,
    indices_buffer: wgpu::Buffer,

    depth_texture: Texture,

    bind_groups: Vec<wgpu::BindGroup>,
    _sets: Vec<RenderSet>,
}

impl Renderer {
    pub fn new(
        state: &HardwareState, 
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        bind_groups: Vec<wgpu::BindGroup>,
        sets: Vec<RenderSet>,
        shader: &Shader,
    ) -> Self {
        let layout = state.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        // let vertex_layouts = [Vertex::desc(), QuadInstance::desc()];
        let vertex_layouts = [Vertex::desc()];

        let vertices_buffer = state.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Vertices Buffer"),
            contents: bytemuck::cast_slice(&QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let indices_buffer = state.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Indices Buffer"),
            contents: bytemuck::cast_slice(&QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let depth_texture = Texture::create_depth_texture(state);

        let render_pipeline = state.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState { 
                module: shader.module(),
                entry_point: shader.vertex_entry(), 
                buffers: &vertex_layouts, // todo: add support of multiple instance buffers for
                                          // dynamic render distance
            },
            fragment: Some(wgpu::FragmentState { 
                module: shader.module(), 
                entry_point: shader.fragment_entry(), 
                targets: &[Some(wgpu::ColorTargetState { 
                    format: *state.surface_format(),
                    blend: Some(wgpu::BlendState::REPLACE), 
                    write_mask: wgpu::ColorWrites::ALL
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
            depth_stencil: Some(wgpu::DepthStencilState { 
                format: Texture::DEPTH_FORMAT, 
                depth_write_enabled: true, 
                depth_compare: wgpu::CompareFunction::Less, 
                // todo: read more on these two
                stencil: wgpu::StencilState::default(), 
                bias: wgpu::DepthBiasState::default()
            }),
            multisample: wgpu::MultisampleState { 
                count: 1, 
                mask: !0, 
                alpha_to_coverage_enabled: false
            },
            multiview: None,
        });

        Self {
            render_pipeline,
            vertices_buffer,
            indices_buffer,
            depth_texture,
            bind_groups,
            _sets: sets,
        }
    }

    pub fn render(&self, state: &HardwareState) -> Result<(), wgpu::SurfaceError> {
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
                                r: 0.03, 
                                g: 0.04, 
                                b: 0.1, 
                                a: 1.0 
                            }
                        ), 
                        store: true
                    }
                })], 
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: self.depth_texture.view(), 
                    depth_ops: Some(wgpu::Operations { 
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: true 
                    }),
                    stencil_ops: None
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            
            for bind_group in self.bind_groups.iter() {
                render_pass.set_bind_group(0, bind_group, &[]);
            }

            render_pass.set_vertex_buffer(0, self.vertices_buffer.slice(..));
            render_pass.set_index_buffer(self.indices_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // for (i, render_set) in self.sets.iter().enumerate() {
            //     render_pass.set_vertex_buffer(i + 1, render_set.instances_buffer().slice(..));
            // }

            render_pass.draw_indexed(0..QUAD_INDICES.len() as _, 0, 0..6000);
        }

        state.queue().submit(std::iter::once(encoder.finish()));
        texture.present();

        Ok(())
    }

    pub fn resize(&mut self, state: &HardwareState, size: winit::dpi::PhysicalSize<u32>) {
        self.depth_texture.resize_texture(state, size);
    }
}

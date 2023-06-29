use wgpu::util::DeviceExt;

use crate::{HardwareState, Shader, Vertex, Descriptable, QUAD_INDICES, QUAD_VERTICES, Texture, Chunk};


pub enum PipelineType {
    Triangle,
    Line,
}

impl PipelineType {
    pub fn toggle(&mut self) {
        match self {
            Self::Triangle => *self = Self::Line,
            Self::Line     => *self = Self::Triangle,
        }
    }
}

pub struct Renderer {
    render_pipeline: wgpu::RenderPipeline,
    line_render_pipeline: wgpu::RenderPipeline,

    ui_render_pipeline: wgpu::RenderPipeline,

    vertices_buffer: wgpu::Buffer,
    indices_buffer: wgpu::Buffer,

    sample_count: u32,
    ms_texture: Texture,
    depth_texture: Texture,

    bind_groups: Vec<wgpu::BindGroup>,
    active_pipeline: PipelineType,
}

impl Renderer {
    fn create_render_pipeline(
        state: &HardwareState, 
        layout: &wgpu::PipelineLayout,
        vertex_layouts: &[wgpu::VertexBufferLayout],
        pipeline_descriptor: &wgpu::RenderPipelineDescriptor,
    ) -> wgpu::RenderPipeline {
        let render_pipeline = state.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                buffers: &vertex_layouts,
                ..pipeline_descriptor.vertex.clone()
            },
            ..pipeline_descriptor.clone()
        });

        render_pipeline
    }

    fn create_wireframe_pipeline(
        state: &HardwareState,
        layout: &wgpu::PipelineLayout,
        vertex_layouts: &[wgpu::VertexBufferLayout],
        pipeline_descriptor: &wgpu::RenderPipelineDescriptor,
        fragment_state: &wgpu::FragmentState,
    ) -> wgpu::RenderPipeline {
        let line_render_pipeline = state.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                buffers: &vertex_layouts,
                ..pipeline_descriptor.vertex.clone()
            },
            fragment: Some(wgpu::FragmentState {
                entry_point: "line_frag",
                ..fragment_state.clone()
            }),
            primitive: wgpu::PrimitiveState {
                polygon_mode: wgpu::PolygonMode::Line,
                ..pipeline_descriptor.primitive.clone()
            },
            ..pipeline_descriptor.clone()
        });

        line_render_pipeline
    }

    fn create_ui_pipeline(
        state: &HardwareState,
        ui_shader: &Shader,
        pipeline_descriptor: &wgpu::RenderPipelineDescriptor,
    ) -> wgpu::RenderPipeline {
        let ui_render_pipeline = state.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Render Pipeline"),
            vertex: wgpu::VertexState {
                module: ui_shader.module(),
                entry_point: ui_shader.vertex_entry(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: ui_shader.module(),
                entry_point: ui_shader.fragment_entry(),
                targets: &[Some(wgpu::ColorTargetState { 
                    format: state.surface_format().clone(),
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent { 
                            src_factor: wgpu::BlendFactor::SrcAlpha, 
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha, 
                            operation: wgpu::BlendOperation::Add 
                        },
                        alpha: wgpu::BlendComponent::OVER,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            ..pipeline_descriptor.clone()
        });

        ui_render_pipeline
    }

    pub fn new(
        state: &HardwareState, 
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        bind_groups: Vec<wgpu::BindGroup>,
        shader: &Shader,
        ui_shader: &Shader,
        sample_count: u32,
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

        let depth_texture = Texture::create_depth_texture(state, sample_count);

        let fragment_targets = [Some(wgpu::ColorTargetState { 
            format: *state.surface_format(),
            blend: Some(wgpu::BlendState::REPLACE), 
            write_mask: wgpu::ColorWrites::ALL
        })];
        let fragment_state = wgpu::FragmentState {
            module: shader.module(),
            entry_point: shader.fragment_entry(),
            targets: &fragment_targets,
        };
        let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState { 
                module: shader.module(),
                entry_point: shader.vertex_entry(), 
                buffers: &[], // todo: add support of multiple instance buffers for
                                          // dynamic render distance
            },
            fragment: Some(fragment_state.clone()),
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
                count: sample_count, 
                mask: !0, 
                alpha_to_coverage_enabled: false
            },
            multiview: None,
        };
        
        let ms_texture = Texture::create_texture(state, state.window().inner_size(), state.surface_format().clone(), sample_count, Some("MS texture"));
        
        let render_pipeline = Self::create_render_pipeline(state, &layout, &vertex_layouts, &pipeline_descriptor);

        let line_render_pipeline = Self::create_wireframe_pipeline(state, &layout, &vertex_layouts, &pipeline_descriptor, &fragment_state);

        let ui_render_pipeline = Self::create_ui_pipeline(state, ui_shader, &pipeline_descriptor);

        Self {
            render_pipeline,
            line_render_pipeline,
            ui_render_pipeline,
            vertices_buffer,
            indices_buffer,
            sample_count,
            ms_texture,
            depth_texture,
            bind_groups,
            active_pipeline: PipelineType::Triangle,
        }
    }

    fn get_active_pipeline(&self) -> &wgpu::RenderPipeline {
        match self.active_pipeline {
            PipelineType::Triangle => &self.render_pipeline,
            PipelineType::Line     => &self.line_render_pipeline,
        }
    }

    pub fn toggle_pipeline(&mut self) {
        self.active_pipeline.toggle();
    }

    pub fn render(&self, state: &HardwareState, chunks: &[&Chunk]) -> Result<(), wgpu::SurfaceError> {
        let mut encoder = state.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        let texture = state.surface().get_current_texture()?;
        let view = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                label: Some("Render Pass"), 
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: if self.sample_count == 1 { &view } else { self.ms_texture.view() }, 
                    resolve_target: if self.sample_count == 1 { None } else { Some(&view) }, 
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

            // block rendering
            render_pass.set_pipeline(self.get_active_pipeline());
            
            for (index, bind_group) in self.bind_groups.iter().enumerate() {
                render_pass.set_bind_group((index + 1) as _, bind_group, &[]);
            }

            render_pass.set_vertex_buffer(0, self.vertices_buffer.slice(..));
            render_pass.set_index_buffer(self.indices_buffer.slice(..), wgpu::IndexFormat::Uint16);

            for chunk in chunks {
                render_pass.set_bind_group(0, chunk.bind_group(), &[]);
                render_pass.draw_indexed(0..QUAD_INDICES.len() as _, 0, 0..chunk.face_count());
            }

            // UI rendering
            render_pass.set_pipeline(&self.ui_render_pipeline);

            render_pass.set_vertex_buffer(0, self.vertices_buffer.slice(..));
            render_pass.set_index_buffer(self.indices_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..QUAD_INDICES.len() as _, 0, 0..6);
        }

        state.queue().submit(std::iter::once(encoder.finish()));
        texture.present();

        Ok(())
    }

    pub fn resize(&mut self, state: &HardwareState, size: winit::dpi::PhysicalSize<u32>) {
        self.depth_texture.resize_texture(state, size);
        self.ms_texture.resize_texture(state, size);
    }
}

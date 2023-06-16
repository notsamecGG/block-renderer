use crate::HardwareState;

pub struct Shader {
    module: wgpu::ShaderModule,

    vertex_entry: &'static str,
    fragment_entry: &'static str,
}

impl Shader {
    pub fn module(&self) -> &wgpu::ShaderModule {
        &self.module
    }

    pub fn vertex_entry(&self) -> &str {
        self.vertex_entry
    }

    pub fn fragment_entry(&self) -> &str {
        self.fragment_entry
    }
}

impl Shader {
    pub fn new(
        state: &HardwareState, 
        path: &str, 
        vertex_entry: &str, 
        fragment_entry: &str, 
        vertex_buffers_layout: [wgpu::VertexBufferLayout],
        label: Option<&str>
    ) -> Self {

        // todo: implement dynamic refreshing of the shader with use of the notify library
        let module = state.device().create_shader_module(&wgpu::ShaderModuleDescriptor { 
            label, 
            source: wgpu::ShaderSource::Wgsl(std::fs::read_to_string(path).unwrap().into()),
        });

        Self {
            module,
            vertex_entry,
            fragment_entry,
        }
    }
}

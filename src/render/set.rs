use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::HardwareState;



pub trait Descriptable {
    const STEP_MODE: wgpu::VertexStepMode;
    const SIZE: wgpu::BufferAddress;

    fn attribs() -> &'static [wgpu::VertexAttribute];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: Self::SIZE,
            step_mode: Self::STEP_MODE,
            attributes: Self::attribs(),
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadInstance { }

impl Descriptable for QuadInstance {
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Instance;
    const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;

    fn attribs() -> &'static [wgpu::VertexAttribute] {
        &wgpu::vertex_attr_array![]
    }
}


pub struct RenderSet {
    instances: Vec<QuadInstance>,
    instances_queue: Vec<QuadInstance>,
    instances_size: usize,
    
    state_ref: Rc<HardwareState>,

    instances_buffer: wgpu::Buffer,
}

impl RenderSet {
    pub fn instances(&self) -> &[QuadInstance] {
        &self.instances
    }

    pub fn instances_len(&self) -> usize {
        self.instances.len()
    }

    pub fn instances_buffer(&self) -> &wgpu::Buffer {
        &self.instances_buffer
    }
}

impl RenderSet {
    const INSTANCES_DESCRIPTOR: wgpu::util::BufferInitDescriptor<'static> = wgpu::util::BufferInitDescriptor {
        label: Some("RenderSet Instance Buffer"),
        contents: &[],
        usage: wgpu::BufferUsages::VERTEX,
    };

    pub fn new(
        instances: Vec<QuadInstance>,
        state_ref: Rc<HardwareState>,
    ) -> Self {
        let instances_buffer = state_ref.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(&instances),
                ..Self::INSTANCES_DESCRIPTOR
            }
        );

        let instances_queue = Vec::new();
        let instances_size = instances.len();

        Self { 
            instances,
            instances_queue,
            instances_size,
            state_ref: state_ref.clone(),
            instances_buffer,
        }
    }

    pub fn set_instances(&mut self, instances: Vec<QuadInstance>) -> Result<(), &str> {
        if instances.len() > self.instances_size {
            return Err("New instances length does not match old instances length");
        }

        self.instances = instances;
        self.state_ref.queue().write_buffer(
            &self.instances_buffer,
            0,
            bytemuck::cast_slice(&self.instances),
        );

        Ok(())
    }

    pub fn remove_instance(&mut self, index: usize) {
        self.instances.remove(index);
    }

    pub fn add_instance(&mut self) {
        let instance = QuadInstance {};
        self.instances_queue.push(instance);
    }

    pub fn add_instances(&mut self, number: usize) {
        let mut instances = vec![QuadInstance {}; number];
        self.instances_queue.append(&mut instances);
    }

    /// Instances may be removed from the memory at any time,
    /// this syncs the buffer with the memory
    pub fn update_instances(&mut self) -> Result<(), &str> {
        if self.instances.len() > self.instances_size {
            return Err("New instances length does not match old instances length");
        }

        self.state_ref.queue().write_buffer(
            &self.instances_buffer,
            0,
            bytemuck::cast_slice(&self.instances),
        ); 

        Ok(())
    }

    /// It is intended to run only when necessary, 
    /// because it is creating a whole new buffer 
    pub fn submit_instances(&mut self) -> Result<(), &str> {
        if self.instances_queue.is_empty() {
            return Err("No instances to submit");
        }

        // update the instances
        let new_instances = &mut self.instances;
        new_instances.append(&mut self.instances_queue);

        self.instances_size = new_instances.len();

        // swap the buffers
        self.instances_buffer.destroy();
        self.instances_buffer = self.state_ref.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(&new_instances),
                ..Self::INSTANCES_DESCRIPTOR
            }
        );

        Ok(())
    }
}

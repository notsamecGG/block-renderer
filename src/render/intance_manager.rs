use bytemuck::Zeroable;
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
pub struct Instance { }

impl Descriptable for Instance {
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Instance;
    const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;

    fn attribs() -> &'static [wgpu::VertexAttribute] {
        &wgpu::vertex_attr_array![]
    }
}


pub struct InstanceManager {
    instances: Vec<Instance>,
    instance_count: usize,

    instance_buffer_delta: usize,

    buffer: Option<wgpu::Buffer>,
    buffer_count: usize,
}

impl InstanceManager {
    const INSTANCES_DESCRIPTOR: wgpu::util::BufferInitDescriptor<'static> = wgpu::util::BufferInitDescriptor {
        label: Some("InstanceManager Instance Buffer"),
        contents: &[],
        usage: wgpu::BufferUsages::VERTEX,
    };

    pub fn new(instance_count: usize) -> Self {
        let instances = vec![Instance::zeroed(); instance_count];

        Self {
            instances,
            instance_count,

            instance_buffer_delta: instance_count,

            buffer: None,
            buffer_count: 0,
        }
    }

    pub fn set_instances(&mut self, state: &HardwareState, instance_count: usize) {
        if instance_count == 0  || (instance_count == self.instance_count  &&  self.buffer.is_some()) {
            return;
        }

        let instances = vec![Instance::zeroed(); instance_count];
        self.buffer = Some(state.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(&instances),
                ..Self::INSTANCES_DESCRIPTOR
            }
        ));

        self.instances = instances;
        self.instance_count = instance_count;
        self.buffer_count = instance_count;
        self.instance_buffer_delta = 0;
    }

    pub fn add_instances(&mut self, state: &HardwareState, instance_count: usize) {
        self.set_isntances(state, self.instance_count + instance_count)
    }

    pub fn remove_instances(&mut self, state: &HardwareState, instance_count: usize) {
        self.set_isntances(state, self.instance_count - instance_count)
    }
}

use crate::{Vertex, QuadInstance};

#[derive(Clone, Copy)]
pub struct RenderSet {
    vertices: [Vertex],
    indices: [u16],
    instances: [QuadInstance],
}

impl RenderSet {
    pub fn new(
        vertices: [Vertex],
        indices: [u16],
        instances: [QuadInstance],
    ) -> Self {

        Self { 
            vertices, 
            indices, 
            instances,
        }
    }

    pub fn set_instances(&mut self, instances: [QuadInstance]) {
        self.instances = instances;
    }
}

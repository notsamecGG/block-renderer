use crate::Vertex;


pub const QUAD_VERTICES: [Vertex; 4] = [
    Vertex { position: [0.0, 0.0, 0.0] },
    Vertex { position: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, 0.0] },
    Vertex { position: [0.0, 1.0, 0.0] },
];

pub const QUAD_INDICES: [u16; 6] = [
    0, 1, 2,
    0, 2, 3
];


// pub const QUAD: RenderSet = RenderSet::new(
//     QUAD_VERTICES, 
//     QUAD_INDICES, 
//     1
// );
//
// pub const FRONT_FACE: RenderSet = QUAD;
//
//
// const BACK_VERTICES: [Vertex; 4] = [
//     Vertex::new(0.0, 0.0, -1.0),
//     Vertex::new(1.0, 0.0, -1.0),
//     Vertex::new(1.0, 1.0, -1.0),
//     Vertex::new(0.0, 1.0, -1.0),
// ];
//
// const BACK_INDICES: [u16; 6] = [
//     0, 2, 1,
//     0, 3, 2
// ];
//
// pub const BACK_FACE: RenderSet = RenderSet::new(
//     BACK_VERTICES, 
//     BACK_INDICES, 
//     1
// );
//
//
// const LEFT_VERTICES: [Vertex; 4] = [
//     Vertex::new(0.0, 0.0, 0.0),
//     Vertex::new(0.0, 0.0, -1.0),
//     Vertex::new(0.0, 1.0, -1.0),
//     Vertex::new(0.0, 1.0, 0.0),
// ];
//
// const LEFT_INDICES: [u16; 6] = [
//     0, 2, 1,
//     0, 3, 2
// ];
//
// pub const LEFT_FACE: RenderSet = RenderSet::new(
//     LEFT_VERTICES, 
//     LEFT_INDICES, 
//     1
// );
//
//
// const RIGHT_VERTICES: [Vertex; 4] = [
//     Vertex::new(1.0, 0.0, 0.0),
//     Vertex::new(1.0, 0.0, -1.0),
//     Vertex::new(1.0, 1.0, -1.0),
//     Vertex::new(1.0, 1.0, 0.0),
// ];
//
// const RIGHT_INDICES: [u16; 6] = [
//     0, 1, 2,
//     0, 2, 3
// ];
//
// pub const RIGHT_FACE: RenderSet = RenderSet::new(
//     RIGHT_VERTICES, 
//     RIGHT_INDICES, 
//     1
// );
//
//
// const TOP_VERTICES: [Vertex; 4] = [
//     Vertex::new(0.0, 1.0, 0.0),
//     Vertex::new(1.0, 1.0, 0.0),
//     Vertex::new(1.0, 1.0, -1.0),
//     Vertex::new(0.0, 1.0, -1.0),
// ];
//
// const TOP_INDICES: [u16; 6] = [
//     0, 1, 2,
//     0, 2, 3
// ];
//
// pub const TOP_FACE: RenderSet = RenderSet::new(
//     TOP_VERTICES, 
//     TOP_INDICES, 
//     1
// );
//
//
// const BOTTOM_VERTICES: [Vertex; 4] = [
//     Vertex::new(0.0, 0.0, 0.0),
//     Vertex::new(1.0, 0.0, 0.0),
//     Vertex::new(1.0, 0.0, -1.0),
//     Vertex::new(0.0, 0.0, -1.0),
// ];
//
// const BOTTOM_INDICES: [u16; 6] = [
//     0, 2, 1,
//     0, 3, 2
// ];
//
// pub const BOTTOM_FACE: RenderSet = RenderSet::new(
//     BOTTOM_VERTICES, 
//     BOTTOM_INDICES, 
//     1
// );
// ];
//

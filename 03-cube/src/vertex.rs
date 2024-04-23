use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2]
}

impl Vertex {
    pub fn new(pos: [i8; 3], tc: [i8; 2]) -> Self {
        Vertex {
            _pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
            _tex_coord: [tc[0] as f32, tc[1] as f32]
        }
    }
}

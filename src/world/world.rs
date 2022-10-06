// Define world vertex structure
#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32)
}

implement_vertex!(Vertex, position);

pub const VERTICES: [Vertex; 3] = [
    Vertex { position: (0.0, 0.5, 0.0) },
    Vertex { position: (0.5, -0.5, 0.0) },
    Vertex { position: (-0.5, -0.5, 0.0) },
];

// Define world indice structure
pub const INDICES: [u16; 3] = [0, 1, 2];

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

// Define world normals
pub const NORMALS: [Normal; 3] = [
    Normal { normal: (0.0, 0.0, 0.0) },
    Normal { normal: (0.0, 0.0, 0.0) },
    Normal { normal: (0.0, 0.0, 0.0) },
];
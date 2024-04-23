mod vertex;

use vertex::Vertex;

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let vertex_data = [
        // top (0, 0, 1)
        Vertex::new([-1, -1, 1], [0, 0]),
        Vertex::new([1, -1, 1], [1, 0]),
        Vertex::new([1, 1, 1], [1, 1]),
        Vertex::new([-1, 1, 1], [0, 1]),
        // bottom (0, 0, -1)
        Vertex::new([-1, 1, -1], [1, 0]),
        Vertex::new([1, 1, -1], [0, 0]),
        Vertex::new([1, -1, -1], [0, 1]),
        Vertex::new([-1, -1, -1], [1, 1]),
        // right (1, 0, 0)
        Vertex::new([1, -1, -1], [0, 0]),
        Vertex::new([1, 1, -1], [1, 0]),
        Vertex::new([1, 1, 1], [1, 1]),
        Vertex::new([1, -1, 1], [0, 1]),
        // left (-1, 0, 0)
        Vertex::new([-1, -1, 1], [1, 0]),
        Vertex::new([-1, 1, 1], [0, 0]),
        Vertex::new([-1, 1, -1], [0, 1]),
        Vertex::new([-1, -1, -1], [1, 1]),
        // front (0, 1, 0)
        Vertex::new([1, 1, -1], [1, 0]),
        Vertex::new([-1, 1, -1], [0, 0]),
        Vertex::new([-1, 1, 1], [0, 1]),
        Vertex::new([1, 1, 1], [1, 1]),
        // back (0, -1, 0)
        Vertex::new([1, -1, 1], [0, 0]),
        Vertex::new([-1, -1, 1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([1, -1, -1], [0, 1]),
        ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data.to_vec(), index_data.to_vec())   
}


fn main() {
    todo!();
}

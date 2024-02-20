use super::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: [Vertex; 3],
}

impl Polygon {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self { vertices }
    }
}

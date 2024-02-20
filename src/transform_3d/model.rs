use super::{face::Face, polygon::Polygon, vertex::Vertex};

#[derive(Debug, Clone)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub polygons: Vec<Polygon>,
}

impl Model {
    pub fn new(vertices: Vec<Vertex>, faces: Vec<Face>) -> Self {
        let mut polygons: Vec<Polygon> = vec![];
        for face in faces.iter() {
            let v: [Vertex; 3] = [
                vertices[(face.x - 1) as usize].clone(),
                vertices[(face.y - 1) as usize].clone(),
                vertices[(face.z - 1) as usize].clone(),
            ];
            polygons.push(Polygon::new(v));
        }
        Self {
            vertices,
            faces,
            polygons,
        }
    }
}

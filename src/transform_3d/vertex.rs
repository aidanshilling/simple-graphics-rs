use nalgebra::Vector4;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vertex { x, y, z }
    }
    pub fn to_vector(self) -> Vector4<f64> {
        return Vector4::new(self.x, self.y, self.z, 1.0);
    }
}

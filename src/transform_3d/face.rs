#[derive(Debug, Clone)]
pub struct Face {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Face {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Face { x, y, z }
    }
}

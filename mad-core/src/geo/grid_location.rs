#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GridLocation {
    pub x: f64,
    pub y: f64,
}

impl GridLocation {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

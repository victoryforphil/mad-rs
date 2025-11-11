#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridCell {
    pub id_x: u8,
    pub id_y: u8,
}

impl GridCell {
    pub fn new(id_x: u8, id_y: u8) -> Self {
        Self { id_x, id_y }
    }
}

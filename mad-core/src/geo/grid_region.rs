#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridRegion {
    pub id: u8,
}

impl GridRegion {
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    /// Creates a new GridRegion from x,y coordinates within a zone
    /// Columns: 8 letters per zone (A-H, J-R, or S-Z)
    /// Rows: 20 letters (A to V, omitting I and O)
    pub fn new_xy(x: u8, y: u8) -> Self {
        Self { id: x * 20 + y }
    }
}

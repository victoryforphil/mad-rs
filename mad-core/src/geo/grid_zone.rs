

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridZone{
    pub id: u8, // 0-59
}

impl GridZone{
    pub fn new(id: u8) -> Self{
        Self{ id }
    }
}
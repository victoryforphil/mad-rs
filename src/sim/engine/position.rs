///
//. Position
///
/// A position in 3D space
/// 
/// # Fields
/// 
/// * `x` - The x coordinate
/// * `y` - The y coordinate
/// * `z` - The z coordinate
/// 
/// # Example
/// 
/// ```
/// 

pub struct Position{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position{
    pub fn new(x:f32,y:f32,z:f32) -> Position{
        Position{
           x,y,z
        }
    }

    pub fn zero() -> Position{
        Self::new(0.0, 0.0, 0.0)
    }

    
}
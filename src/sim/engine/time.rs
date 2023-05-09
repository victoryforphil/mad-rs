/// Time struct
/// 
/// Stores timing information for the engine
/// 
/// # Fields
/// * `delta_time` - The time since the last frame
/// * `time` - The time since the start of the program
/// 
/// # Example
/// 
/// ```
/// let time = MADTime::new();
/// ```
pub struct MADTime{
    pub delta_time: f32,
    pub time: f32,
}

impl MADTime{
    pub fn new() -> MADTime{
        MADTime{
            delta_time: 0.0,
            time: 0.0,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_time(){
        let time = MADTime::new();
        assert_eq!(time.delta_time, 0.0);
        assert_eq!(time.time, 0.0);
    }
}
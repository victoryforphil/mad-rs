use log::info;

use super::{time::MADTime, position::Position};

pub trait MADEntity{
    fn update(&mut self, delta_time: &MADTime);
    fn init(&self);
    ///
    /// Sets the position of the entity in world space
    /// 
    /// # Arguments
    /// 
    /// * `position` - The position to set
    fn set_position(&mut self, position: Position);

    ///
    /// Gets the position of the entity in world space
    /// 
    /// # Returns
    /// 
    /// * `&Position` - The position of the entity
    fn get_position(&self) -> &Position;

    ///
    /// Gets the bounds of the entity (width, height) for collision / selection
    /// 
    /// # Returns
    /// 
    /// * `(f32, f32)` - The bounds of the entity
    fn get_bounds(&self) -> (f32, f32);
    fn get_sprite_id(&self) -> String;
}


/// Test entity for unit testing
/// 
/// # Fields
/// 
/// * `test_state` - A test state
/// 

pub struct MADTestEntity{
    pub test_state: u32,
    position: Position,
}

impl MADEntity for MADTestEntity{
    fn update(&mut self, _delta_time: &MADTime){
        self.test_state += 1;

        if self.test_state > 100{
            self.test_state = 0;
            info!("[MADTestEntity] Resetting test state");
        }
    }

    fn init(&self) {
        info!("[MADTestEntity] Initializing test entity");
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_bounds(&self) -> (f32, f32) {
        (10.0, 10.0) // 10m x 10m
    }

    fn get_sprite_id(&self) -> String {
        "unit-silo".to_string() 
    }

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_entity(){
        let mut entity = MADTestEntity{test_state: 0, position: Position::zero()};
        let time = MADTime::new();

        assert_eq!(entity.test_state, 0);

        entity.update(&time);
        assert_eq!(entity.test_state, 1);


        for _ in 0..100{
            entity.update(&time);
        }
        assert_eq!(entity.test_state, 0);
    }
}
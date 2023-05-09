use log::info;

use super::time::MADTime;

pub trait MADEntity{
    fn update(&mut self, delta_time: &MADTime);
    fn init(&self);
}


/// Test entity for unit testing
/// 
/// # Fields
/// 
/// * `test_state` - A test state
/// 

pub struct MADTestEntity{
    pub test_state: u32,
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

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_entity(){
        let mut entity = MADTestEntity{test_state: 0};
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
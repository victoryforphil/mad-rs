use std::collections::HashMap;

use log::info;

use super::{entity::MADEntity, time::MADTime, world_entity::MADRenderContext};


///
/// World struct
/// 
/// Stores entities and handles their updates and rendering
/// 
/// # Fields
/// 
/// * `entities` - A hashmap of entities
/// * `entity_idx` - The latest entity index (private)
/// 
/// # Example
/// 
/// ```
/// let world = MADWorld::new();
/// 
/// let entity = Box::new(MADTestEntity::new());
/// world.add_entity(entity);
/// 
/// ```
pub struct MADWorld{
    pub entities: HashMap<u32, Box<dyn MADEntity>>,
    // Private
    entity_idx: u32, // Latest entity index
}

impl MADWorld{
    pub fn new() -> MADWorld{
        MADWorld{
            entities: HashMap::new(),
            entity_idx: 0,
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn MADEntity>){
        info!("[MADWorld] Adding entity with index {}", self.entity_idx);
        self.entities.insert(self.entity_idx, entity);
        self.entity_idx += 1;
    }

    pub fn get_entity(&self, idx: u32) -> Option<&Box<dyn MADEntity>>{
        self.entities.get(&idx)
    }

    pub fn update(&mut self, delta_time: &MADTime){
        for (_, entity) in &mut self.entities{
            entity.update(delta_time);
        }
    }

    pub fn render(&self){
        for (_, entity) in &self.entities{
           
        }
    }
}


// TESTS
#[cfg(test)]
mod tests{
    use crate::sim::engine::entity::MADTestEntity;

    use super::*;

    #[test]
    fn test_world_new(){
        let mut world = MADWorld::new();
        let time = MADTime::new();

        assert_eq!(world.entities.len(), 0);
    }

    #[test]
    fn test_world_add_entity(){
        let mut world = MADWorld::new();
        let time = MADTime::new();

        let entity = Box::new(MADTestEntity{test_state: 0, position: Position::zero()});
        world.add_entity(entity);

        assert_eq!(world.entities.len(), 1);
    }

    #[test]
    fn test_world_get_entity(){
        let mut world = MADWorld::new();
        let time = MADTime::new();

        let entity = Box::new(MADTestEntity{test_state: 0, position: Position::zero()});
        world.add_entity(entity);

        let entity = world.get_entity(0).unwrap();
        assert_eq!(entity.test_state, 0);
    }

    
}
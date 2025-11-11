use std::collections::HashMap;

use crate::system::ECSSystem;
use crate::timing::ECSTimingSystem;
use log::*;
pub type ECSSystemHandle = Box<dyn ECSSystem>;

pub struct ECSSystemRunner{
    pub systems: HashMap<String, ECSSystemHandle>,
    pub timing_system: ECSTimingSystem,
}

impl ECSSystemRunner{
    pub fn new() -> Self{
        Self{ systems: HashMap::new(), timing_system: ECSTimingSystem::new(24) }
    }

    pub fn add_system(&mut self, system: ECSSystemHandle) -> Result<(), anyhow::Error>{
        debug!("Added system: {}", system.get_name());
        self.systems.insert(system.get_name(), system); 
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), anyhow::Error>{
        for (name, system) in self.systems.iter_mut(){
            system.execute(&HashMap::new())?;
            trace!("Executed system: {}", name);
        }
        let wait_time = self.timing_system.tick();
        trace!("Waiting for next tick, wait time: {}", wait_time);
        if wait_time > 0 {
            std::thread::sleep(std::time::Duration::from_millis(wait_time));
        }
      
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::system::ECSSystemMock;

    #[test]
    fn test_new(){
        let runner = ECSSystemRunner::new();
        assert_eq!(runner.systems.len(), 0);
        assert_eq!(runner.timing_system.goal_tick_rate, 24);
    }

    #[test]
    fn test_add_system(){
        let mut runner = ECSSystemRunner::new();
        let system = Box::new(ECSSystemMock::new());
        runner.add_system(system).unwrap();
        assert_eq!(runner.systems.len(), 1);
        assert!(runner.systems.contains_key("test"));
    }

    #[test]
    fn test_add_multiple_systems(){
        let mut runner = ECSSystemRunner::new();
        
        let mut system1 = ECSSystemMock::new();
        system1.name = "system1".to_string();
        runner.add_system(Box::new(system1)).unwrap();
        
        let mut system2 = ECSSystemMock::new();
        system2.name = "system2".to_string();
        runner.add_system(Box::new(system2)).unwrap();
        
        assert_eq!(runner.systems.len(), 2);
        assert!(runner.systems.contains_key("system1"));
        assert!(runner.systems.contains_key("system2"));
    }

    #[test]
    fn test_execute_no_systems(){
        let mut runner = ECSSystemRunner::new();
        let result = runner.execute();
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_with_system(){
        let mut runner = ECSSystemRunner::new();
        let system = Box::new(ECSSystemMock::new());
        runner.add_system(system).unwrap();
        
        let result = runner.execute();
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_multiple_systems(){
        let mut runner = ECSSystemRunner::new();
        
        let mut system1 = ECSSystemMock::new();
        system1.name = "system1".to_string();
        runner.add_system(Box::new(system1)).unwrap();
        
        let mut system2 = ECSSystemMock::new();
        system2.name = "system2".to_string();
        runner.add_system(Box::new(system2)).unwrap();
        
        let result = runner.execute();
        assert!(result.is_ok());
    }

    #[test]
    fn test_timing_system_ticks(){
        let mut runner = ECSSystemRunner::new();
        let initial_tick_count = runner.timing_system.tick_count;
        
        runner.execute().unwrap();
        
        // Tick count should have incremented or rate should be calculated
        assert!(runner.timing_system.tick_count > initial_tick_count || 
                runner.timing_system.current_tick_rate > 0);
    }
}
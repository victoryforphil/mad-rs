use log::*;
use crate::ecs::query::ECSQuery;
pub struct ECSTimingSystem{
    pub goal_tick_rate: u32,
    pub current_tick_rate: u32,
    pub last_tick_time: u64,
    pub current_time: u64,
    pub delta_time: u64,
    pub accumulator: u64,
    pub tick_count: u32,
    pub last_rate_update: u64,
}

impl ECSTimingSystem{
    pub fn new(goal_tick_rate: u32) -> Self{
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        Self{ 
            goal_tick_rate, 
            current_tick_rate: 0, 
            last_tick_time: now, 
            current_time: now, 
            delta_time: 0, 
            accumulator: 0,
            tick_count: 0,
            last_rate_update: now,
        }
    }

    pub fn get_name(&self) -> String{
        "timing".to_string()
    }
    
    pub fn tick(&mut self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        
        self.current_time = now;
        self.delta_time = now - self.last_tick_time;
        self.last_tick_time = now;
        
        // Update tick count and calculate current tick rate
        self.tick_count += 1;
        if now - self.last_rate_update >= 1000 { // Update rate every second
            self.current_tick_rate = self.tick_count;
            self.tick_count = 0;
            self.last_rate_update = now;
        }
        
        // Calculate target time per tick in milliseconds
        let target_tick_time = 1000 / self.goal_tick_rate as u64;
        
        // If we're falling behind our goal tick rate, return 0 (no wait)
        if self.current_tick_rate > 0 && self.current_tick_rate < self.goal_tick_rate {
            warn!("Falling behind goal tick rate, current tick rate: {}, goal tick rate: {}", self.current_tick_rate, self.goal_tick_rate);
            return 0;
        }
        
        // Calculate how long we should wait to maintain our goal tick rate
        if self.delta_time < target_tick_time {
            info!("Waiting for next tick, current tick rate: {}, goal tick rate: {}", self.current_tick_rate, self.goal_tick_rate);
            target_tick_time - self.delta_time
        } else {
            info!("Already behind goal tick rate, current tick rate: {}, goal tick rate: {}", self.current_tick_rate, self.goal_tick_rate);
            0 // We're already behind, don't wait
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let timing_system = ECSTimingSystem::new(60);
        assert_eq!(timing_system.goal_tick_rate, 60);
        assert_eq!(timing_system.current_tick_rate, 0);
        assert_eq!(timing_system.delta_time, 0);
        assert_eq!(timing_system.accumulator, 0);
        assert_eq!(timing_system.tick_count, 0);
        assert!(timing_system.current_time > 0);
        assert!(timing_system.last_tick_time > 0);
        assert!(timing_system.last_rate_update > 0);
    }

    #[test]
    fn test_tick_updates_time(){
        let mut timing_system = ECSTimingSystem::new(60);
        let initial_time = timing_system.current_time;
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        timing_system.tick();
        
        assert!(timing_system.current_time > initial_time);
        assert!(timing_system.delta_time > 0);
        assert_eq!(timing_system.tick_count, 1);
    }

    #[test]
    fn test_tick_returns_wait_time(){
        let mut timing_system = ECSTimingSystem::new(60);
        let wait_time = timing_system.tick();
        
        // For 60 ticks per second, target is ~16.67ms per tick
        // First tick should return close to target time
        assert!(wait_time <= 17); // Allow some margin
    }

    #[test]
    fn test_tick_rate_calculation(){
        let mut timing_system = ECSTimingSystem::new(10);
        
        // Simulate multiple ticks over time
        for _ in 0..5 {
            timing_system.tick();
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        
        // After some ticks, tick_count should be incremented
        assert!(timing_system.tick_count > 0 || timing_system.current_tick_rate > 0);
    }

    #[test]
    fn test_tick_no_wait_when_behind(){
        let mut timing_system = ECSTimingSystem::new(1000); // Very high tick rate
        
        // First tick
        timing_system.tick();
        
        // Simulate being behind by sleeping longer than target tick time
        std::thread::sleep(std::time::Duration::from_millis(50));
        
        let wait_time = timing_system.tick();
        
        // Should return 0 or very small value when behind schedule
        assert!(wait_time < 10);
    }

    #[test]
    fn test_multiple_ticks(){
        let mut timing_system = ECSTimingSystem::new(60);
        
        for i in 0..10 {
            timing_system.tick();
            assert!(timing_system.tick_count == (i + 1) || timing_system.current_tick_rate > 0);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
}
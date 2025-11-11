use std::collections::HashMap;

use crate::command::ECSCommandType;
use crate::component::{self, ECSComponent};
use crate::query::ECSQueryType;
use crate::system::ECSSystem;
use crate::timing::ECSTimingSystem;
use log::*;
use mad_datastore::{Datastore, EntityQuery};
use mad_datastore::mad_core::geo::GridRegion;
pub type ECSSystemHandle = Box<dyn ECSSystem>;

pub struct ECSSystemRunner {
    pub systems: HashMap<String, ECSSystemHandle>,
    pub timing_system: ECSTimingSystem,
    pub command_queue: Vec<ECSCommandType>,
    pub datastore: Datastore,
    pub region: GridRegion,
}

impl ECSSystemRunner {
    pub fn new(region: GridRegion) -> Self {
        Self {
            systems: HashMap::new(),
            timing_system: ECSTimingSystem::new(24),
            command_queue: Vec::new(),
            datastore: Datastore::new(region),
            region: region,
        }
    }

    pub fn add_system(&mut self, system: ECSSystemHandle) -> Result<(), anyhow::Error> {
        debug!("Added system: {}", system.get_name());
        self.systems.insert(system.get_name(), system);
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), anyhow::Error> {
        for (name, system) in self.systems.iter_mut() {
            let queries = system.get_queries();
            let mut components = HashMap::new();
            for query in queries {
                match query.query {
                    ECSQueryType::Component(component_name) => {
                        let table = self.datastore.get_entity(component_name).unwrap();
                        let rows =
                            table.query(EntityQuery::All);
                        for row in rows {
                            let component =
                                ECSComponent::from_row(row.clone(), component_name.clone());
                            components.insert(component.name.clone(), component);
                        }
                    }
                }
            }
            system.execute(&components, &mut self.command_queue)?;
            trace!(
                "Executed system: {} with {} components",
                name,
                components.len()
            );
        }
        let wait_time = self.timing_system.tick();
        trace!("Waiting for next tick, wait time: {}", wait_time);
        if wait_time > 0 {
            std::thread::sleep(std::time::Duration::from_millis(wait_time));
        }

        Ok(())
    }

    fn process_command(&mut self, command: ECSCommandType) -> Result<(), anyhow::Error> {
        match command {
            ECSCommandType::SpawnEntity(components) => {
                debug!("Spawning entity with {} components", components.len());
                let entity_index = self.datastore.get_entity_index();
                for component in components {
                    let table = self.datastore.get_entity(component.name.clone()).unwrap();
                    table.append_row(EntityRow::new(self.region.cell, entity_index));
                }
            }
            ECSCommandType::AppendToEntity(entity_index, components) => {}
            ECSCommandType::RemoveFromEntity(entity_index, components) => {}
        }
        Ok(())
    }
}
    #[test]
        let mut runner = ECSSystemRunner::new(GridRegion::new(0));
        let system = Box::new(ECSSystemMock::new());
        runner.add_system(system).unwrap();
        assert_eq!(runner.systems.len(), 1);
        assert!(runner.systems.contains_key("test"));
    }

    #[test]
    fn test_add_multiple_systems() {
        let mut runner = ECSSystemRunner::new(GridRegion::new(0));

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
    fn test_execute_no_systems() {
        let mut runner = ECSSystemRunner::new(GridRegion::new(0));
        let result = runner.execute();
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_with_system() {
        let mut runner = ECSSystemRunner::new(GridRegion::new(0));
        let system = Box::new(ECSSystemMock::new());
        runner.add_system(system).unwrap();

        let result = runner.execute();
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_multiple_systems() {
        let mut runner = ECSSystemRunner::new(GridRegion::new(0));

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
    fn test_timing_system_ticks() {
        let mut runner = ECSSystemRunner::new();
        let initial_tick_count = runner.timing_system.tick_count;

        runner.execute().unwrap();

        // Tick count should have incremented or rate should be calculated
        assert!(
            runner.timing_system.tick_count > initial_tick_count
                || runner.timing_system.current_tick_rate > 0
        );
    }
}

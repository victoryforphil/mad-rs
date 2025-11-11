use anyhow;
use mad_engine::command::ECSCommandType;
use mad_engine::component::ECSComponent;
use mad_engine::system::ECSSystem;
use std::collections::HashMap;

pub struct SysAircraftMove {
    pub name: String,
}

impl SysAircraftMove {
    pub fn new() -> Self {
        Self {
            name: "AircraftMovement".to_string(),
        }
    }
}

impl ECSSystem for SysAircraftMove {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_queries(&mut self) -> Vec<mad_engine::ecs::query::ECSQuery> {
        Vec::new()
    }

    fn execute(&mut self, _components: &HashMap<String, ECSComponent>, _commands: &mut Vec<ECSCommandType>) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

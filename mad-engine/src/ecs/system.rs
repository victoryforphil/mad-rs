use std::{collections::HashMap, sync::mpsc::Sender};

use mad_datastore::StoredType;

use crate::{EventEntry, command::ECSCommandType, component::ECSComponent, ecs::query::ECSQuery};

use log::*;

// ----- Trait for ECS Systems -----
pub trait ECSSystem {
    fn get_name(&self) -> String;
    fn get_queries(&mut self) -> Vec<ECSQuery>;
    fn execute(
        &mut self,
        components: &HashMap<String, ECSComponent>,
        commands: &mut Vec<ECSCommandType>,
    ) -> Result<(), anyhow::Error>;
}

// ----- Mock System for testing -----
pub struct ECSSystemMock {
    pub name: String,
    pub queries: Vec<ECSQuery>,
    pub last_result: Result<(), anyhow::Error>,
    pub last_components: HashMap<String, ECSComponent>,
}

impl ECSSystemMock {
    pub fn new() -> Self {
        Self {
            name: "test".to_string(),
            queries: Vec::new(),
            last_result: Ok(()),
            last_components: HashMap::new(),
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ECSSystem for ECSSystemMock {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_queries(&mut self) -> Vec<ECSQuery> {
        self.queries.clone()
    }

    fn execute(&mut self, components: &HashMap<String, ECSComponent>) -> Result<(), anyhow::Error> {
        debug!("Executing system with queries: {:?}", self.queries);
        self.last_result = Ok(());
        self.last_components = components.clone();
        Ok(())
    }
}
// ----- Tests -----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut system = ECSSystemMock::new();
        assert_eq!(system.get_name(), "test");
        assert_eq!(system.get_queries().len(), 0);
        assert!(system.last_result.is_ok());
        assert!(system.last_components.is_empty());
    }

    #[test]
    fn test_execute() {
        let mut system = ECSSystemMock::new();
        system.execute(&HashMap::new()).unwrap();
        assert!(system.last_result.is_ok());
        assert!(system.last_components.is_empty());
    }

    #[test]
    fn test_get_queries() {
        let mut system = ECSSystemMock::new();
        assert_eq!(system.get_name(), "test");
        system.get_queries();
        assert_eq!(system.get_queries().len(), 0);
    }

    #[test]
    fn test_execute_with_components() {
        let mut system = ECSSystemMock::new();
        assert_eq!(system.get_name(), "test");
        system.execute(&HashMap::new()).unwrap();
        assert!(system.last_result.is_ok());
        assert!(system.last_components.is_empty());
    }
}

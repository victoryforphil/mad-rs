use std::collections::HashMap;

use mad_datastore::StoredType;

use crate::{ecs::query::ECSQuery, EventEntry};

use log::*;

// ----- Trait for ECS Systems -----
pub trait ECSSystem{
    fn get_queries(&mut self) -> Vec<ECSQuery>;
    fn execute(&mut self, components: &HashMap<String, Vec<StoredType>>) -> Result<(), anyhow::Error>;
}

// ----- Mock System for testing -----
pub struct ECSSystemMock{
    pub queries: Vec<ECSQuery>,
    pub last_result: Result<(), anyhow::Error>,
    pub last_components: HashMap<String, Vec<StoredType>>,
}

impl ECSSystemMock{
    pub fn new() -> Self{
        Self{ queries: Vec::new(), last_result: Ok(()), last_components: HashMap::new() }
    }
}

impl ECSSystem for ECSSystemMock{

    fn get_queries(&mut self) -> Vec<ECSQuery>{
        self.queries.clone()
    }

    fn execute(&mut self, components: &HashMap<String, Vec<StoredType>>) -> Result<(), anyhow::Error>{
        debug!("Executing system with queries: {:?}", self.queries);
        self.last_result = Ok(());
        self.last_components = components.clone();
        Ok(())
    }
}
// ----- Tests -----
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let mut system = ECSSystemMock::new();
        assert_eq!(system.get_queries().len(), 0);
        assert!(system.last_result.is_ok());
        assert!(system.last_components.is_empty());
    }

    #[test]
    fn test_execute(){
        let mut system = ECSSystemMock::new();
        system.execute(&HashMap::new()).unwrap();
        assert!(system.last_result.is_ok());
        assert!(system.last_components.is_empty());
    }

    #[test]
    fn test_get_queries(){
        let mut system = ECSSystemMock::new();
        system.get_queries();
        assert_eq!(system.get_queries().len(), 0);
    }

    #[test]
    fn test_execute_with_components(){
        let mut system = ECSSystemMock::new();
        system.execute(&HashMap::new()).unwrap();
        assert!(system.last_result.is_ok());
        assert!(system.last_components.is_empty());
    }
}
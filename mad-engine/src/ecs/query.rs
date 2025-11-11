use mad_datastore::mad_core::geo::GridLocation;

    #[derive(Debug, Clone, PartialEq)]
pub enum ECSQueryType{
    Component(String),
}
#[derive(Debug, Clone, PartialEq)]
pub struct ECSQuery{
    pub query: ECSQueryType,
    pub grid_location: GridLocation,
}

impl ECSQuery{
    pub fn new(query: ECSQueryType, grid_location: GridLocation) -> Self{
        Self{ query, grid_location }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let query = ECSQuery::new(ECSQueryType::Component("test".to_string()), GridLocation::new(0.0, 0.0));
        assert_eq!(query.query, ECSQueryType::Component("test".to_string()));
        assert_eq!(query.grid_location, GridLocation::new(0.0, 0.0));
    }
}
use std::collections::HashMap;

use log::debug;
use mad_core::geo::GridRegion;

use crate::{EntityTable, LayerTable};

#[derive(Debug)]
pub struct Datastore {
    pub region: GridRegion,
    layers: HashMap<String, LayerTable>,
    entities: HashMap<String, EntityTable>,
    entity_index: u32,
    layer_index: u32,
}

impl Datastore {
    pub fn new(region: GridRegion) -> Self {
        Self {
            region,
            layers: HashMap::new(),
            entities: HashMap::new(),
            entity_index: 0,
            layer_index: 0,
        }
    }

    pub fn get_entity_index(&mut self) -> u32 {
        self.entity_index += 1;
        self.entity_index
    }

    pub fn get_layer_index(&mut self) -> u32 {
        self.layer_index += 1;
        self.layer_index
    }

    pub fn get_layer(&mut self, layer_type: String) -> Result<&LayerTable, anyhow::Error> {
        // If no table is found create it
        if !self.layers.contains_key(&layer_type) {
            debug!(
                "Layer type {} not found, creating new layer table",
                layer_type
            );
            let layer_table = LayerTable::new(layer_type.clone());
            self.layers.insert(layer_type.clone(), layer_table);
        }
        self.layers
            .get(&layer_type)
            .ok_or(anyhow::anyhow!("Layer type {} not found", layer_type))
    }

    pub fn get_layer_mut(&mut self, layer_type: String) -> Result<&mut LayerTable, anyhow::Error> {
        if !self.layers.contains_key(&layer_type) {
            debug!(
                "Layer type {} not found, creating new layer table",
                layer_type
            );
            let layer_table = LayerTable::new(layer_type.clone());
            self.layers.insert(layer_type.clone(), layer_table);
        }
        self.layers
            .get_mut(&layer_type)
            .ok_or(anyhow::anyhow!("Layer type {} not found", layer_type))
    }

    pub fn get_entity(&mut self, entity_type: String) -> Result<&EntityTable, anyhow::Error> {
        if !self.entities.contains_key(&entity_type) {
            debug!(
                "Entity type {} not found, creating new entity table",
                entity_type
            );
            let entity_table = EntityTable::new(entity_type.clone());
            self.entities.insert(entity_type.clone(), entity_table);
        }
        self.entities
            .get(&entity_type)
            .ok_or(anyhow::anyhow!("Entity type {} not found", entity_type))
    }

    pub fn get_entity_mut(
        &mut self,
        entity_type: String,
    ) -> Result<&mut EntityTable, anyhow::Error> {
        if !self.entities.contains_key(&entity_type) {
            debug!(
                "Entity type {} not found, creating new entity table",
                entity_type
            );
            let entity_table = EntityTable::new(entity_type.clone());
            self.entities.insert(entity_type.clone(), entity_table);
        }
        self.entities
            .get_mut(&entity_type)
            .ok_or(anyhow::anyhow!("Entity type {} not found", entity_type))
    }
}

#[cfg(test)]
mod tests {
    use mad_core::geo::GridCell;

    use crate::LayerRow;

    use super::*;

    #[test]
    fn test_new() {
        let datastore = Datastore::new(GridRegion::new(0));
        assert_eq!(datastore.region, GridRegion::new(0));
    }

    #[test]
    fn test_get_layer() {
        let mut datastore = Datastore::new(GridRegion::new(0));
        let layer_table = datastore.get_layer("test_layer".to_string()).unwrap();
        assert_eq!(layer_table.type_name(), "test_layer");
        assert_eq!(layer_table.rows().len(), 0);
    }

    #[test]
    fn test_get_layer_mut() {
        let mut datastore = Datastore::new(GridRegion::new(0));
        let layer_table = datastore.get_layer_mut("test_layer".to_string()).unwrap();
        assert_eq!(layer_table.type_name(), "test_layer");
        assert_eq!(layer_table.rows().len(), 0);
    }

    #[test]
    fn test_get_entity() {
        let mut datastore = Datastore::new(GridRegion::new(0));
        let entity_table = datastore.get_entity("test_entity".to_string()).unwrap();
        assert_eq!(entity_table.type_name(), "test_entity");
        assert_eq!(entity_table.rows().len(), 0);
    }

    #[test]
    fn test_get_entity_mut() {
        let mut datastore = Datastore::new(GridRegion::new(0));
        let entity_table = datastore.get_entity_mut("test_entity".to_string()).unwrap();
        assert_eq!(entity_table.type_name(), "test_entity");
        assert_eq!(entity_table.rows().len(), 0);
    }
}

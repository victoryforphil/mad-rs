use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

use mad_core::geo::GridCell;

use crate::StoredType;

#[derive(Clone)]
pub struct EntityRow {
    pub cell: GridCell,
    pub index: u32,
    pub entity_index: u32,
    pub fields: HashMap<String, StoredType>,
}

impl Debug for EntityRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EntityRow{{ cell: {:?}, index: {}, entity_index: {}, fields: {{",
            self.cell, self.index, self.entity_index
        )?;
        for (key, value) in &self.fields {
            write!(f, "{}: {:?}, ", key, value)?;
        }
        write!(f, "}} }}")
    }
}

impl EntityRow {
    pub fn new(cell: GridCell, entity_index: u32) -> Self {
        Self {
            cell,
            index: 0,
            entity_index,
            fields: HashMap::new(),
        }
    }
    pub fn get_field(&self, key: &str) -> Option<&StoredType> {
        self.fields.get(key)
    }
    pub fn set_field(&mut self, key: String, value: impl Into<StoredType>) {
        self.fields.insert(key, value.into());
    }
}

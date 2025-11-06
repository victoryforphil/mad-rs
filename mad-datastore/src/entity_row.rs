use std::{collections::HashMap, fmt::{Debug, Formatter}};

use mad_core::geo::GridCell;

use crate::StoredType;

pub struct EntityRow{
    pub cell: GridCell,
    pub index: u32,
    pub entity_index: u32,
    pub fields: HashMap<String, StoredType>,
}

impl Debug for EntityRow{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EntityRow{{ cell: {:?}, index: {}, entity_index: {}, fields: {{", self.cell, self.index, self.entity_index)?;
        for (key, value) in &self.fields {
            write!(f, "{}: {:?}, ", key, value)?;
        }
        write!(f, "}} }}")
    }
}
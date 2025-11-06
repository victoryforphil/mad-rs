use std::{collections::HashMap, fmt::{Debug, Formatter}};

use mad_core::geo::GridCell;

use crate::StoredType;




pub struct LayerRow{
    pub cell: GridCell,
    pub index: u32,
    pub fields: HashMap<String, StoredType>,
}

impl Debug for LayerRow{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LayerRow{{ cell: {:?}, index: {}, fields: {{", self.cell, self.index)?;
        for (key, value) in &self.fields {
            write!(f, "{}: {:?}, ", key, value)?;
        }
        write!(f, "}} }}")
    }
}

impl LayerRow{
    pub fn new(cell: GridCell) -> Self{
        Self{ cell, index: 0, fields: HashMap::new() }
    }
    pub fn get_field(&self, key: &str) -> Option<&StoredType>{
        self.fields.get(key)
    }
    pub fn set_field(&mut self, key: String, value: StoredType){
        self.fields.insert(key, value);
    }
}


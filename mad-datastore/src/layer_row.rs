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



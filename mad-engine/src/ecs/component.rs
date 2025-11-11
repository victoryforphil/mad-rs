use std::collections::HashMap;

use mad_datastore::StoredType;

#[derive(Debug, Clone, PartialEq)]
pub struct ECSComponent {
    pub name: String,
    pub fields: HashMap<String, StoredType>,
}

impl ECSComponent {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: HashMap::new(),
        }
    }

    pub fn get_field(&self, key: &str) -> Option<&StoredType> {
        self.fields.get(key)
    }

    pub fn set_field(&mut self, key: String, value: impl Into<StoredType>) {
        self.fields.insert(key, value.into());
    }

    pub fn from_row(row: EntityRow, component_type: String) -> Self {
        let mut component = Self::new(component_type);
        for (key, value) in row.fields.iter() {
            component.set_field(key.clone(), value.clone());
        }
        component
    }
}

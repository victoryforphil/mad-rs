use std::collections::HashMap;

use mad_datastore::StoredType;

#[derive(Debug, Clone)]
pub struct EventEntry{
    pub event_type: String,
    pub event_data: StoredType
}

impl EventEntry{
    pub fn new(event_type: String, event_data: StoredType) -> Self{
        Self{ event_type, event_data }
    }
}
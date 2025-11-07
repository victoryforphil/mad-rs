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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let event_entry = EventEntry::new("test".to_string(), StoredType::new("test".to_string(), vec![1, 2, 3]));
        assert_eq!(event_entry.event_type, "test");
        assert_eq!(event_entry.event_data.key, "test");
        assert_eq!(event_entry.event_data.data, vec![1, 2, 3]);
    }
}
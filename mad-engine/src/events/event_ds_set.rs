use mad_datastore::StoredType;

use crate::{EventBuilder, EventEntry};

pub struct DSSetEventBuilder {}

impl EventBuilder for DSSetEventBuilder {
    fn build(&self) -> EventEntry {
        EventEntry::new(
            "DS_SET".to_string(),
            StoredType {
                key: "SET_PAYLOAD".to_string(),
                data: vec![],
            },
        )
    }
}

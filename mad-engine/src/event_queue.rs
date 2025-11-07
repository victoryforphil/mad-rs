use crate::EventEntry;


#[derive(Debug)]
pub struct EventQueue{
    events: Vec<EventEntry>,
}

impl EventQueue{
    pub fn new() -> Self{
        Self{ events: Vec::new() }
    }

    pub fn add_event(&mut self, event: EventEntry){
        self.events.push(event);
    }

    pub fn pop_event(&mut self) -> Option<EventEntry>{
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }

    pub fn queue_size(&self) -> usize{
        self.events.len()
    }

    pub fn is_empty(&self) -> bool{
        self.events.is_empty()
    }

    pub fn clear(&mut self){
        self.events.clear();
    }
}

#[cfg(test)]
mod tests{
    use mad_datastore::StoredType;

    use super::*;

    #[test]
    fn test_new(){
        let event_queue = EventQueue::new();
        assert_eq!(event_queue.events.len(), 0);
    }

    #[test]
    fn test_add_event(){
        let mut event_queue = EventQueue::new();
        event_queue.add_event(EventEntry::new("test".to_string(), StoredType::default()));
        assert_eq!(event_queue.events.len(), 1);
    }

    #[test]
    fn test_pop_event(){
        let mut event_queue = EventQueue::new();
        event_queue.add_event(EventEntry::new("test".to_string(), StoredType::default()));
        let event = event_queue.pop_event().unwrap();
        assert_eq!(event.event_type, "test");
        assert_eq!(event.event_data.key, "NULL");
        assert_eq!(event.event_data.data, vec![]);
    }

    #[test]
    fn test_queue_size(){
        let event_queue = EventQueue::new();
        assert_eq!(event_queue.queue_size(), 0);
    }

    #[test]
    fn test_is_empty(){
        let event_queue = EventQueue::new();
        assert_eq!(event_queue.is_empty(), true);
    }

    #[test]
    fn test_clear(){
        let mut event_queue = EventQueue::new();
        event_queue.add_event(EventEntry::new("test".to_string(), StoredType::default()));
        event_queue.clear();
        assert_eq!(event_queue.events.len(), 0);
    }
}
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
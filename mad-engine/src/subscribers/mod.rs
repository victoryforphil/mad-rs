use crate::EventEntry;

pub trait Subscriber {
    fn get_subscription(&self) -> &str;
    fn on_event(&self, event: &EventEntry) -> Result<(), anyhow::Error>;
}

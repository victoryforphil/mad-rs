use crate::EventEntry;
pub mod event_ds_set;
pub use event_ds_set::*;

pub trait EventBuilder {
    fn build(&self) -> EventEntry;
}

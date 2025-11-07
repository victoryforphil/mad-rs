pub trait Subscriber{
    fn get_subscription(&self) -> &str;
    fn on_event(&self, event: &Event) -> Result<(), Error>;
}
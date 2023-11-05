use super::Message;
use iced_futures::futures::channel::mpsc as iced_mpsc;
use iced_futures::subscription::Subscription;
pub struct Flags {
    pub recipe: Box<dyn Fn() -> Subscription<Message>>,
}
impl Flags {
    pub fn new(recipe: Box<dyn Fn() -> Subscription<Message>>) -> Self {
        Self { recipe }
    }
}

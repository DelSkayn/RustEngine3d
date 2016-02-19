
use super::event_queue::EventQueue;

pub struct Communication{
    events: EventQueue,
}

impl Communication{
    pub fn new() -> Self{
        Communication{
            events: EventQueue::new(),
        }
    }
}

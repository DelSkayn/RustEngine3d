


use super::Entity;
use super::Generation;

pub struct Entities{
    next: u32,
}

impl Entities{
    pub fn new() -> Self{
        Entities{
            next: 0,
        }
    }

    pub fn create(&mut self) -> Entity{
        let id = self.next;
        self.next += 1;
        Entity(Generation(0),id)
    }
}


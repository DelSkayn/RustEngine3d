use std::sync::atomic::AtomicBool;

use super::Game;

pub struct Root<G: Game + ?Sized>{
    pub game: G,
    pub running: AtomicBool,
}

impl<G: Game + ?Sized> Root<G>{
    pub fn new() -> Self{
        info!("Root created.");
        Root{
            running: AtomicBool::new(true),
            game: G::new(),
        }
    }
}

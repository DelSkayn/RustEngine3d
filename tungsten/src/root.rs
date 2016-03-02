use std::sync::atomic::AtomicBool;

use super::Game;
use super::Platform;

pub struct Root<G: Game + ?Sized>{
    pub platform: Platform,
    pub game: G,
    pub running: AtomicBool,
}

impl<G: Game + ?Sized> Root<G>{
    pub fn new() -> Self{
        info!("Root created.");
        Root{
            running: AtomicBool::new(true),
            game: G::new(),
            platform: Platform::new(),
        }
    }
}

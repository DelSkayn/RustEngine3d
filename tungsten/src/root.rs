use super::Game;
use super::Communication;
use super::Platform;

pub struct Root<G: Game + ?Sized>{
    pub game: G,
    pub platform: Platform,
    pub comms: Communication,
}

impl<G: Game + ?Sized> Root<G>{
    pub fn new() -> Self{
        info!("Root created.");
        Root{
            game: G::new(),
            platform: Platform::new(),
            comms: Communication::new(),
        }
    }
}

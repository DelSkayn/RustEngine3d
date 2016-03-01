use super::Game;
use super::Communication;

pub struct Root<G: Game + ?Sized>{
    game: G,
    comms: Communication,
}

impl<G: Game + ?Sized> Root<G>{
    pub fn new() -> Self{
        info!("Root created.");
        Root{
            game: G::new(),
            comms: Communication::new(),
        }
    }
}

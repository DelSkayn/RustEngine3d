use super::Game;
use super::Communication;

pub struct Root<G: Game>{
    game: G,
    comms: Communication,
}

impl<G: Game> Root<G>{
    pub fn new() -> Self{
        info!("Root created.");
        Root{
            game: G::new(),
            comms: Communication::new(),
        }
    }
}


use super::Game;
use super::Platform;
use super::Settings;

use super::util::Running;

///
///Root is a static data structure used throughout the engine
///for communication. Root can only be referenced as constant.
///So referies need to ensure that data can be accessed as 
///internaly mutable.
///
pub struct Root{
    /// Information about the platform the engine is running on.
    pub platform: Platform,
    /// An object used to determin if the object should continue running.
    pub running: Running, 
    pub game: Box<Game>,
}

pub struct SyncRoot{
    /// Object of the game the engine is running.
    /// Settings of versious things in the engine
    pub settings: Settings,
}

impl Root{
    /// creates a new root.
    pub fn new<G: Game + Sized + 'static>(game: G) -> Self{
        info!("Root created.");
        Root{
            game: Box::new(game),
            platform: Platform::new(),
            running: Running::new(),
        }
    }
}

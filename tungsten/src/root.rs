
use super::Game;
use super::Platform;
use super::Settings;
use super::IOData;

use std::sync::Arc;

use super::render::RenderRoot;

use super::util::Running;

///
///Root is a static data structure used throughout the engine
///for communication. Root can only be referenced as constant.
///So referies need to ensure that data can be accessed as 
///internaly mutable.
///
pub struct Root{
    pub async: Arc<AsyncRoot>,
    pub sync: SyncRoot,
}

pub struct SyncRoot{
    /// Object of the game the engine is running.
    pub game: Box<Game>,
    /// Settings of versious things in the engine
    pub settings: Settings,

    pub io: IOData,
}

pub struct AsyncRoot{
    /// Information about the platform the engine is running on.
    pub platform: Platform,
    /// Data used by rendering engine and everyone who needs to submit renderdata.
    pub render: RenderRoot,
    /// An object used to determin if the object should continue running.
    pub running: Running, 
}

impl Root{
    /// creates a new root.
    pub fn new<G: Game + Sized + 'static>(game: G) -> Self{
        info!("Root created.");
        Root{
            sync: SyncRoot{
                game: Box::new(game),
                settings: Settings::new(),
                io: IOData::new(),
            },
            async: Arc::new(AsyncRoot{
                platform: Platform::new(),
                running: Running::new(),
                render: RenderRoot::new(),
            }),
        }
    }
}

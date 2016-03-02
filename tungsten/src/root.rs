use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use super::Game;
use super::Platform;
use super::Settings;

pub struct Root{
    pub platform: Platform,
    pub game: Box<Game>,
    pub running: Running, 
    pub settings: Settings,
}

pub struct Running{
    interal: AtomicBool,
}

impl Running{
    fn new() -> Self{
        Running{
            interal: AtomicBool::new(true),
        }
    }

    pub fn quit(&self){
        self.interal.store(false,Ordering::Relaxed);
    }

    pub fn should(&self) -> bool{
        self.interal.load(Ordering::Relaxed)
    }
}

impl Root{
    pub fn new<G: Game + Sized + 'static>(game: G) -> Self{
        info!("Root created.");
        Root{
            running: Running::new(),
            game: Box::new(game),
            platform: Platform::new(),
            settings: Settings::new(),
        }
    }
}

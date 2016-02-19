#![crate_name = "tungsten"]
#![crate_type = "lib"]
#![allow(dead_code)]

#[macro_use]
extern crate log as log_ext;

extern crate time;

mod log;
use log::SimpleLogger;

mod game;
pub use game::Game;

mod root;
pub use root::Root;

mod event;
pub use event::Event;

mod kernal;
use kernal::Kernal;

mod communication;
pub use communication::Communication;

mod schedular;
mod event_queue;

pub struct Engine;

impl Engine{
    pub fn go<G: Game>(){
        SimpleLogger::init().unwrap();
        info!("Engine starting.");
        let mut root = Root::<G>::new();
        let mut kernal = Kernal::new(&mut root);
        kernal.run();
        info!("Engine closed.");
    }
}

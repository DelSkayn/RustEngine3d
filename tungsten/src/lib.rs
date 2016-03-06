//!
//! Tungsten
//! ========
//!
//! Tungsten is a game engine written as a future proof, game engine.
//!
#![crate_name = "tungsten"]
#![crate_type = "lib"]
#![allow(dead_code)]
#![deny(missing_docs)]

#[macro_use]
extern crate log as log_ext;
extern crate time;
extern crate crossbeam;

mod log;
use log::SimpleLogger;

mod game;
pub use game::Game;

mod root;
pub use root::Root;
pub use root::AtomicOption;

mod platform;
pub use platform::Platform;

mod settings;
pub use settings::Settings;

mod event;
pub use event::Event;

mod kernal;
pub use kernal::System;
pub use kernal::Schedular;
pub use kernal::Job;
use kernal::Kernal;

mod window;
use window::WindowSystem;

mod render;
use render::RenderSystem;

mod event_queue;

pub struct Engine;

impl Engine{
    pub fn go<G: Game + 'static>(game: G){
        println!("--------------------------------------------------------------------------");
        println!(r#"   ______                                        __                       
  /\__  _\                                      /\ \__                    
  \/_/\ \/   __  __    ___       __       ____  \ \ ,_\     __     ___    
     \ \ \  /\ \/\ \ /' _ `\   /'_ `\    /',__\  \ \ \/   /'__`\ /' _ `\  
      \ \ \ \ \ \_\ \/\ \/\ \ /\ \L\ \  /\__, `\  \ \ \_ /\  __/ /\ \/\ \ 
       \ \_\ \ \____/\ \_\ \_\\ \____ \ \/\____/   \ \__\\ \____\\ \_\ \_\
        \/_/  \/___/  \/_/\/_/ \/___L\ \ \/___/     \/__/ \/____/ \/_/\/_/
                                 /\____/                                  
                                 \_/__/                        "#);
        println!("--------------------------------------------------------------------------");
        println!("Tungsten starting!");
        SimpleLogger::init().unwrap();
        let mut root = Root::new(game);
        let window = WindowSystem::new(&root);
        let mut kernal = Kernal::new(&mut root);
        kernal.add_system(Box::new(window));
        kernal.run();
        info!("Engine closed.");
    }
}

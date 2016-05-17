//!
//! Tungsten
//! ========
//!
//! Tungsten is a game engine written as a future proof, game engine.
//!
#![crate_name = "tungsten"]
#![crate_type = "lib"]
#![allow(dead_code)]

#![plugin(serde_macros)]
#![feature(custom_derive,plugin)]

extern crate serde;
extern crate serde_json;
//#![deny(missing_docs)]

//uuhhhh
//I hate that it needs to be declared here.
#[macro_use]
extern crate gfx;

#[macro_use]
extern crate log as log_ext;
#[macro_use]
extern crate lazy_static;
extern crate time;
extern crate crossbeam;

mod util;

mod log;
use log::SimpleLogger;

mod game;
pub use game::Game;

mod root;
pub use root::Root;

mod platform;
pub use platform::Platform;

mod settings;
pub use settings::Settings;

//mod event;
//pub use event::Event;

mod kernel;
pub use kernel::System;
pub use kernel::Task;
pub use kernel::TaskBuilder;
use kernel::Kernel;

//mod window;
//use window::Window;

//mod render;
//use render::RenderSystem;
//use render::GfxRenderer;

mod io;

const BANNER: &str = r#"
   ______                                        __                       
  /\__  _\                                      /\ \__                    
  \/_/\ \/   __  __    ___       __       ____  \ \ ,_\     __     ___    
     \ \ \  /\ \/\ \ /' _ `\   /'_ `\    /',__\  \ \ \/   /'__`\ /' _ `\  
      \ \ \ \ \ \_\ \/\ \/\ \ /\ \L\ \  /\__, `\  \ \ \_ /\  __/ /\ \/\ \ 
       \ \_\ \ \____/\ \_\ \_\\ \____ \ \/\____/   \ \__\\ \____\\ \_\ \_\
        \/_/  \/___/  \/_/\/_/ \/___L\ \ \/___/     \/__/ \/____/ \/_/\/_/
                                 /\____/                                  
                                 \_/__/                        
"#


pub struct Engine;

impl Engine{
    pub fn go<G: Game + 'static>(game: G){
        println!("--------------------------------------------------------------------------");
        println!(BANNER);
        println!("--------------------------------------------------------------------------");
        println!("Tungsten starting!");
        settings::Settings::set_file("res/settings.json");
        SimpleLogger::init().unwrap();
        let mut kernel = Kernel::new();
        kernel.run();
        info!("Engine closed.");
    }
}

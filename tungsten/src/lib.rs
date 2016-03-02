#![crate_name = "tungsten"]
#![crate_type = "lib"]
#![allow(dead_code)]

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

mod event;
pub use event::Event;

mod kernal;
use kernal::Kernal;

mod event_queue;

pub struct Engine;

impl Engine{
    pub fn go<G: Game>(){
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
        let mut root = Root::<G>::new();
        let mut kernal = Kernal::new(&mut root);
        kernal.run();
        info!("Engine closed.");
    }
}

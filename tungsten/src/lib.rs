#![crate_name = "tungsten"]
#![crate_type = "lib"]
#![allow(dead_code)]


// extern crate serde;
// extern crate serde_json;
//
extern crate time;
extern crate task;
extern crate toml;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;


mod engine;
mod registery;
mod io;
mod window;
mod util;
mod console;
mod render;

pub mod logic;
pub use engine::Engine;
pub use registery::Registery;

pub trait Game {
    fn new() -> Self;
}

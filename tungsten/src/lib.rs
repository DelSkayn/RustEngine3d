#![crate_name = "tungsten"]
#![crate_type = "lib"]
#![allow(dead_code)]

#![plugin(serde_macros)]
#![feature(custom_derive,plugin)]
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate task;
extern crate toml;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;


mod engine;
mod registry;
mod io;
mod window;
mod util;


pub use engine::Engine;
pub use registry::Registry;

pub trait Game{
    fn new() -> Self;
}

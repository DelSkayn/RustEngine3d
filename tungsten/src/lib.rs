#![crate_name = "tungsten"]
#![crate_type = "lib"]
#![allow(dead_code)]

#![plugin(serde_macros)]
#![feature(custom_derive,plugin)]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod settings;
mod engine;
pub use engine::Engine;

trait Game{
    fn init(&mut self);
}

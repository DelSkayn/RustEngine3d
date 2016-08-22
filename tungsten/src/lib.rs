#![crate_name = "tungsten"]
#![crate_type = "lib"]
#![allow(dead_code)]

//! Tungsten game engines.
//! ======================
//!
//! Running the engine
//! ------------------
//!
//! In order to run the engine call the to `Go()` functions. Use the `Game` trait to set the engine
//! in a proper state.
//!


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
#[macro_use]
extern crate glium;

pub mod engine;
pub mod registery;
pub mod window;
pub mod util;
pub mod console;
pub mod render;
pub mod state;

pub mod asset;
pub mod io;
pub mod logic;
pub use engine::Engine;
pub use registery::Registery;
pub use state::State;

pub trait Game {
    fn new() -> Self;
}

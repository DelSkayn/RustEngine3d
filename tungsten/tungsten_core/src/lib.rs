#![crate_name = "tungsten_core"]
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

pub mod registery;
pub mod window;
pub mod util;
pub mod console;
pub mod state;
pub mod io;


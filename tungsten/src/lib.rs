//! This crate combines all the crates that make up the tungsten game engine and exposes the api to
//! the user.
//!
//! # A summary
//!
//! - The `Game` trait is the primary interface int the engine. When creating a game with tungsten
//!   on should implement this trait.
//!
//! - In order to load assets you need to use the `Assets` struct. This struct profides functions
//!   for loading and parsing asset files. Most systems have a dependency on asset as that crate
//!   also profides the formats which the engine uses for assets.
//!
//! - `Console` is used as a debug console and can be used as a simple way to interact with the
//!   engine as it is running. 
//!
//! - `Render` is the rendering engine of tungsten. All render functionality can be found there.
//!
//! - `tungsten_core` provides functionality on which all systems depend. Like `IO` and `Registery`
//!
//! - `Registery` is the setting manager of tungsten. It profides functions for reading config
//!   files and loading settings.
//!
//! - `State` desribes the current state of the engine. It is where the information about wether
//!   the engine is running can be found.
//!
//! - The `Engine` struct contains all the functionality for runnning the engine. The `Go()` function 
//!   which is part of the Enige struct is the starting point of the engine and needs to be called to 
//!   start the engine.
//!
//! - `task-rs` is a multithreading liberary heavily used through out the engine for paralization.
//!   use of this library in game code is encouraged.
//!
//! - All used crates can be accessed by the last part of there name. For instance `tungsten_render` can be
//!   accessed as `tungsten::render`.
//!
//!
//! # Using tungsten.
//!
//!
//! ```rust
//! extern crate tungsten;
//! 
//! use self::tungsten::{Engine,SimpleGame};
//! use self::tungsten::core::State;
//! use self::tungsten::render::Render;
//!
//! struct HelloGame;
//!
//! impl Game for HelloGame{
//!     fn new_simple(_: &mut Render) -> Self{
//!         println!("Game being created!");
//!     }
//!
//!     fn update(&mut self){
//!         println!("Hello from tungsten!");
//!         State::quit();
//!     }
//! }
//!
//! fn main(){
//!     Engine::<HelloGame>::Go();
//! }
//! ```
//!
//!

pub extern crate tungsten_render;
pub extern crate tungsten_core;
pub extern crate tungsten_asset;
pub extern crate tungsten_logic;
pub extern crate task;

#[macro_use]
extern crate log;


mod engine;
pub use engine::*;

mod commands;

pub use self::tungsten_render as render;
pub use self::tungsten_core as core;
pub use self::tungsten_asset as asset;
pub use self::tungsten_logic as logic;

use core::console::{Console,Terminal};

use render::Render;

/// The trait representing a game running on the engine.
pub trait Game{
    /// Called when the engine is started. 
    /// Should return a fully intialized game.
    fn new<T: Terminal>(render: &mut Render,console: &mut Console<T>) -> Self;

    /// Called once a frame should be used to update game logic.
    fn update(&mut self){}
}

/// The trait representing a game running on the engine.
/// This trait does not take a console.
pub trait SimpleGame{
    fn new(render: &mut Render) -> Self;

    fn update(&mut self){}
}

impl<G: SimpleGame> Game for G{
    fn new<T: Terminal>(render: &mut Render,_: &mut Console<T>) -> Self{
        G::new(render)
    }

    fn update(&mut self){
        self.update()
    }
}


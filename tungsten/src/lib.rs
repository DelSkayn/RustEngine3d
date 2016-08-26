extern crate tungsten_render;
extern crate tungsten_core;
extern crate tungsten_asset;
extern crate tungsten_logic;
extern crate task;

mod engine;
pub use engine::*;

pub mod commands;

pub use self::tungsten_render::*;
pub use self::tungsten_core::*;
pub use self::tungsten_asset::{Assets,Mesh,Texture,Material,Container};
pub use self::tungsten_logic::*;

pub use self::tungsten_core::console::{Console,Terminal};

pub trait Game{
    fn new<T: Terminal>(render: &mut Render,console: &mut Console<T>) -> Self;

    fn update(&mut self){}
}


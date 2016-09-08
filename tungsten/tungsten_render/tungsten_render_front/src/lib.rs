//! Tungsten render front
//!
//! This crate contains the implementation of the render front.
//! Its task is the following:
//!
//! - Do visibilty computations, Not implementated
//!
//! - Do interpolation between frames, Not implemented
//!
//! - Copy render data from the external data in to the interal buffer.
//!
//! - Creating a render que from the current data.
//!
//!
#![allow(dead_code)]
extern crate task;
extern crate cgmath;
extern crate tungsten_asset;

use self::task::sync::mutate_inspect::{self,Inspector};
pub use self::task::sync::mutate_inspect::Mutator;

use self::cgmath::*;

use self::tungsten_asset::{Con
/// the renderer trait needs to be implemented by all render backends.
pub trait Renderer {
    fn render(&self);
}

impl<T: Renderer> Renderer for Box<T>{
    fn render(&self){
        self.render();
    }
}

pub struct RenderObject{
    transform: Decomposed<Vector3<f64>,Quarternion<f64>>,
    mesh
}

pub struct Front<T: Render>{
    renderer: T,
    objects: Objects,
}



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
//! Not(!) part of its responisiblities are
//!
//! - Sorting renderque properly.
//!
//! - Caching render data.
//!
#![allow(dead_code)]
#![allow(unused_imports)]
extern crate task;
extern crate cgmath;
extern crate tungsten_asset;

mod objects;
mod format;

use self::task::sync::mutate_inspect::Inspector;
use self::cgmath::*;
use self::tungsten_asset::{Mesh,Container};


use self::objects::StaticObjects;
pub use self::format::*;

/// the renderer trait needs to be implemented by all render backends.
pub trait Renderer {
    fn render(&self,que: &[RenderData]);
}

impl Renderer for Box<Renderer>{
    fn render(&self,que: &[RenderData]){
        (**self).render(que);
    }
}

/// Primary struct for the front end.
/// This struct combines all the functionality into one object.
pub struct Front<T: Renderer>{
    renderer: T,
    objects: StaticObjects,
}

impl<T: Renderer> Front<T>{
    pub fn new(renderer: T) -> Self{
        Front{
            renderer: renderer,
            objects: StaticObjects::new(),
        }
    }

    pub fn add_object(&mut self,object: Inspector<RenderObject>){
        self.objects.add(object);
    }

    pub fn render(&mut self){

    }
}



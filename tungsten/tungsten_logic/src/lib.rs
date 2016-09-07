#![crate_name = "tungsten_logic"]
#![crate_type = "lib"]
#![allow(dead_code)]
#[macro_use]
extern crate log;
extern crate task;

pub mod component;
mod entities;
pub mod system;
mod get_once;

pub use self::component::{Components,Component};
use self::entities::Entities;
pub use self::system::Schedular;

#[derive(Clone,Copy,Eq,PartialEq)]
struct Generation(i32);

type Index = u32;

#[derive(Eq,PartialEq,Clone,Copy)]
pub struct Entity(Generation,Index);

pub struct Logic{
    world: Components,
    entities: Entities,
}

impl Logic{
    pub fn new() -> Self{
        Logic{
            world: Components::new(),
            entities: Entities::new(),
        }
    }

    pub fn components(&mut self) -> &mut Components{
        &mut self.world
    }

    pub fn update<'a,F>(&'a mut self,func: F)
        where F: FnOnce(Schedular<'a>)
    {
        let sched = Schedular::from_component(&self.world);
        func(sched);
    }
}


mod component;
mod entities;
mod system;
mod get_once;

pub use self::system::{Schedular,Args};
pub use self::component::{Component, ComponentStorage,VecStorage};
use self::component::Components;

use self::entities::Entities;

#[derive(Clone,Copy,Eq,PartialEq)]
struct Generation(i32);

type Index = u32;

#[derive(Eq,PartialEq,Clone,Copy)]
struct Entity(Generation,Index);

struct Logic{
    world: Components,
    entities: Entities,
}

impl Logic{
    fn update(&mut self) {}
}

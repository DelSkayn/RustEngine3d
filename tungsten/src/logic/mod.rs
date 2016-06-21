
mod component;
mod entities;
mod system;

pub use self::system::{System,Systems,SystemBuilder,SinkSystem,Args};
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
    systems: Systems,
}

impl Logic{
    fn update(&mut self) {}
}

impl<A:ComponentStorage> System for FnMut(&A) + Sync
    where A: ComponentStorage,
{
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool{
        let a = if let Some(x) = arg.borrow::<A>() { x } else { return false };
        self(&*a);
        true
    }
}

impl<A,B> System for FnMut(&A,&B) + Sync
    where A: ComponentStorage,
          B: ComponentStorage,
{
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool{
        let a = if let Some(x) = arg.borrow::<A>() { x } else { return false };
        let b = if let Some(x) = arg.borrow::<B>() { x } else { return false };
        self(&*a,&*b);
        true
    }
}

impl<A,B,C> System for FnMut(&A,&B,&C) + Sync
    where A: ComponentStorage,
          B: ComponentStorage,
          C: ComponentStorage,
{
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool{
        let a = if let Some(x) = arg.borrow::<A>() { x } else { return false };
        let b = if let Some(x) = arg.borrow::<B>() { x } else { return false };
        let c = if let Some(x) = arg.borrow::<C>() { x } else { return false };
        self(&*a,&*b,&*c);
        true
    }
}

impl<A,B,C,D> System for FnMut(&A,&B,&C,&D) + Sync
    where A: ComponentStorage,
          B: ComponentStorage,
          C: ComponentStorage,
          D: ComponentStorage,
{
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool{
        let a = if let Some(x) = arg.borrow::<A>() { x } else { return false };
        let b = if let Some(x) = arg.borrow::<B>() { x } else { return false };
        let c = if let Some(x) = arg.borrow::<C>() { x } else { return false };
        let d = if let Some(x) = arg.borrow::<D>() { x } else { return false };
        self(&*a,&*b,&*c,&*d);
        true
    }
}

impl<A:ComponentStorage> System for FnMut(&mut A) + Sync
    where A: ComponentStorage,
{
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool{
        let mut a = if let Some(x) = arg.borrow_mut::<A>() { x } else { return false };
        self(&mut *a);
        true
    }
}

impl<A,B> System for FnMut(&mut A,&B) + Sync
    where A: ComponentStorage,
          B: ComponentStorage,
{
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool{
        let mut a = if let Some(x) = arg.borrow_mut::<A>() { x } else { return false };
        let b = if let Some(x) = arg.borrow::<B>() { x } else { return false };
        self(&mut *a,&*b);
        true
    }
}

impl<A,B,C> System for FnMut(&mut A,&B,&C) + Sync
    where A: ComponentStorage,
          B: ComponentStorage,
          C: ComponentStorage,
{
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool{
        let mut a = if let Some(x) = arg.borrow_mut::<A>() { x } else { return false };
        let b = if let Some(x) = arg.borrow::<B>() { x } else { return false };
        let c = if let Some(x) = arg.borrow::<C>() { x } else { return false };
        self(&mut *a,&*b,&*c);
        true
    }
}

impl<A,B,C,D> System for FnMut(&mut A,&B,&C,&D) + Sync
    where A: ComponentStorage,
          B: ComponentStorage,
          C: ComponentStorage,
          D: ComponentStorage,
{
    fn execute<'a>(&'a mut self,arg: Args<'a>) -> bool{
        let mut a = if let Some(x) = arg.borrow_mut::<A>() { x } else { return false };
        let b = if let Some(x) = arg.borrow::<B>() { x } else { return false };
        let c = if let Some(x) = arg.borrow::<C>() { x } else { return false };
        let d = if let Some(x) = arg.borrow::<D>() { x } else { return false };
        self(&mut *a,&*b,&*c,&*d);
        true
    }
}


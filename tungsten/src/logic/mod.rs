
struct Logic;

mod component;

pub use self::component::Component;
pub use self::component::ComponentStorage;

mod generator;

#[derive(Clone,Copy,Eq,PartialEq)]
struct Generation(i32);

type Index = u32;

#[derive(Eq,PartialEq,Clone,Copy)]
struct Entity(Generation,Index);


impl Logic {
    fn update(&mut self) {}
}

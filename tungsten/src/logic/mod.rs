
struct Logic;

mod component;

pub use self::component::Component;
pub use self::component::ComponentStorage;

#[derive(Eq,PartialEq)]
struct Entity(u64);

impl Logic {
    fn update(&mut self) {}
}

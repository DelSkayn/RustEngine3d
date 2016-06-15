use std::any::TypeId;

enum SystemError{
    Again,
}

pub struct RunArgs{
    fn get<T>(self) -> Option<&mut T>
}

enum SystemError{
    Retry,
}

trait System{
    fn execute(&mut self,comp: RunArgs) -> Result<(),SystemError>;
}

impl<T> UpdateSystem for FnMut(&T){
    
}

struct Systems{
    tick: Vec<System>,
}

impl Systems{
    fn run(&mut self){
    }
}

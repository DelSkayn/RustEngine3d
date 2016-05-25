use std::any::TypeId;

enum SystemError{
    Again,
}

trait UpdateSystem{
    fn execute(&mut self,comp: &Components) -> Result<(),SystemError>;
}

impl<T> UpdateSystem for FnMut(&T){
    
}

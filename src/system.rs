use std::collections::HashMap;

trait System{
    fn get_id() -> &'static str;
}

struct SystemHandler{
    systems: HashMap<&'static str,System + Any>,
}

impl SystemHandler{
    pub fn new() -> Self{
        SystemHandler{
            systems: HashMap::new(),
        }
    }

    pub fn add_system<T:System>(sys: T){
        match systems.insert(T:get_id(),sys){
            Some(_) => panic!("Tried to add system which already exists");
            None => {}
        };
    }
}

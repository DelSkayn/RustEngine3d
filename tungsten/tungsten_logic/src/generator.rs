
use std::any::TypeId;
use std::any::Any;

use std::collections::HashMap;

use super::Component;
use super::Entity;
use super::Generation;



struct EntityBuilder{
    components: HashMap<TypeId,Box<Any>>
}

impl EntityBuilder{
    fn new() -> Self{
        EntityBuilder{
            components: HashMap::new(),
        }
    }

    fn with<C: Component>(mut self,comp: C) -> Self{
        let id = TypeId::of::<C>();
        self.components.insert(id,Box::new(comp));
        self
    }
}

pub struct Entities{
}

struct Creator{
    unused: Vec<u32>,
    next_id: u32,
    current_gen: i32,
    should_incr: bool,
}

impl Creator{
    fn new() -> Self{
        Creator{
            unused: Vec::new(),
            next_id: 0,
            current_gen: 1,
            should_incr: false,
        }
    }

    pub fn next(&mut self) -> Entity{
        let id = if !self.unused.is_empty(){
            if self.should_incr{
                self.should_incr = false;
                self.current_gen += 1;
            }
            self.unused.pop().unwrap()
        }else{
            let res = self.next_id;
            self.next_id += 1;
            res
        };
        Entity(Generation(self.current_gen),id)
    }

    pub fn release(&mut self,ent: Entity){
        self.should_incr = true;
        self.unused.push(ent.1);
        self.unused.sort_by(|&a,b| a.cmp(b).reverse());
        self.try_shrink();
    }

    fn try_shrink(&mut self){
        let mut amount = 0;
        for i in 0..self.unused.len(){
            if self.unused[i] == self.next_id - 1{
                amount += 1;
            }else{
                break;
            }
        }
        let len = self.unused.len();
        self.unused.truncate(len - amount);
    }
}

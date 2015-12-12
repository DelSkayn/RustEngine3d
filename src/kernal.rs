use std::collections::HashMap;
use std::iter::Empty;
use std::mem;


use super::Event;

enum SystemOption{
    Syncronized,
    Threaded,
    Optional,
}

trait System{
    fn GetSystemOption() -> SystemOption;
    fn new(handle: &EventHandle) -> Self;
    fn run(&mut self);
}


struct EventQueue{

}

struct EventHandle<'a>{
    queue: &'a EventQueue,
    events: Vec<Event>,
}

impl EventHandle{
    fn pull_events(&mut self){
    }
}

impl<'a> IntoIterator for &'a mut EventHandle{
    type Item = Event;
    type IntoIter = std::vec::IntoIter<Event>;
    
    fn into_iter(self) -> IntoIter{
        let vec = mem::replace(self,Vec::new());
        vec.events.into_iter()
    }
}

struct Kernal{
    running: bool,
    systems: Vec<(&'static str,Box<System>)>,
}

impl Kernal{
    pub fn new() -> Self{
        Kernal{
            systems: HashMap::new(),
        }
    }

    pub fn add_system(&mut self,sys: Box<System>){
        self.systems.push((sys.get_id(),sys));
    }

    pub fn run(&mut self){
        while(self.running){
            //collect
            let events = Empty::<Event>::new();
            for sys in self.systems.iter(){
                events = events.chain(sys.iter());
            }

            for e in events{
                for &mut sys in self.systems.iter(){
                    sys.handel_event(e.clone());
                }
            }
        }
    }
}

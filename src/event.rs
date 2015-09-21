use std::rc::Rc;
use std::cell::RefCell;

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum BaseEvent{
    Null,
    Pause,
    Resume,
    Quit,
    Resize(u32,u32),
    Press(()),
    Mouse(Mouse),
    Release(()),
    Render,
    Physics(f32),
    Update(f32),
}


#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Mouse{
    Move([f32;2]),
    MoveDelta([f32;2]),
    Wheel(i32),
}

pub trait EventCreator<T>{
    fn get_events(&mut self) -> Vec<T>;//TODO: look at better way to implement
}

pub struct EventLoop<T>{
    event_creator: Vec<Rc<RefCell<EventCreator<T>>>>,
    events: Vec<T>,
}

impl<T> EventLoop<T>{
    pub fn new() -> Self{
        EventLoop{
            event_creator: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn pull_events(&mut self){
        self.events.clear();
        for ec in &self.event_creator{
            self.events.extend(ec.borrow_mut().get_events());
        }
    }

    pub fn register(&mut self,ec: Rc<RefCell<EventCreator<T>>>){
        self.event_creator.push(ec);
    }

    pub fn get_events<'a>(&'a self) -> &'a [T]{
        &self.events
    }
}

use std::rc::Rc;

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum BaseEvent{
    Null,
    Pause,
    Resume,
    Quit,
    Resize(u32,u32),
    Mouse(Mouse),
    KeyBoard(KeyBoard),
    Render,
    Physics(f32),
    Update(f32),
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum KeyBoard{
    Pressed(Key),
    Released(Key),
    Character(char),
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Key{
    Key0, Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LAlt,
    RAlt,
    OpenBracket,
    CloseBracket,
    Comma,
    Tab,
    Space,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Up,
    Down,
    Left,
    Right,
    Esc,
    UnkownKey,
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Mouse{
    Move([f32;2]),
    MoveDelta([f32;2]),
    Wheel(f32),
    Pressed(Button),
    Released(Button),
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Button{
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Esc,
    Unkown,
}

pub trait EventCreator<T>{
    fn get_events(&self) -> Vec<T>;//TODO: look at better way to implement
}

pub struct EventLoop<T>{
    event_creator: Vec<Rc<EventCreator<T>>>,
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
            self.events.extend(ec.get_events());
        }
    }

    pub fn register(&mut self,ec: Rc<EventCreator<T>>){
        self.event_creator.push(ec);
    }

    pub fn get_events<'a>(&'a self) -> &'a [T]{
        &self.events
    }
}

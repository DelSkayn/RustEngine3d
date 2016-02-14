#![allow(dead_code)]

//extern crate cgmath;
#[macro_use]
extern crate log;
//general time
extern crate time;

mod kernal;
mod kernal::Error as KError;

//Engine version variables in sync with the one in cargo
const VERSION_MAJOR: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &'static str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &'static str = env!("CARGO_PKG_VERSION_PATCH");

///The Engine the basic struct from which all other data structs spawn
pub struct Engine<G: Game>;

type Win = ();
type Ren = ();
type Res = ();
type Con = ();
type Phy = ();
type Set = ();

enum REError{
    Kernal(KError),
}

impl<G:Game> Engine<G>{

    pub fn Go(){
        let root = Root::<G::GameData>::new();
    }
}


///The root data structure.
///All systems which need to hold onto data between should place it here.
pub struct Root<GameData,>{
    game: GameData,
}

impl Root{
    fn new(){
        Root{
            game: Default,
        }
    }
}

///The trait used when implementing your own game
pub trait Game<'a>{
    type GameData: Default;
    type EventType;

    fn new(data: &'a mut GameData) -> Self<'a>;
    fn update();
    fn render();
    fn init();
}

//The Event 
pub enum Event<GameEvent,InputEvent>{
    Game(GameEvent),
    Input(InputEvent),
}

//Every system which needs to be told to do something
trait System{
    type MessageType;
    
    fn do(&mut self,MessageType);
}

//Every sysetem which needs an Tick.
trait Tick{
    fn run(&mut self);
}

//Every system which wants to follow events
trait Subscriber{
    fn notify(&mut self,event: Event);
}

//Every system which can throw events.
trait Happening<T>{
    fn get_event(&mut self) -> Vec<Event>;
}

//Null systems for use when there is no system in place
//Usefull for debuggin and testing.
struct NullSystem;
struct NullTick;
struct NullSubscriber;
struct NullHappening;

impl System for NullSystem{
    fn do(&mut self,mess: MessageType){}
}

impl Tick for NullTick{
    fn run(&mut self){}
}

impl Subscriber for NullSubscriber{
    fn notify(&mut self,event: Event){}
}

impl Happening for NullHappening{
    fn get_event(&mut self) -> Vec<Event>{
        Vec::new()
    }
}


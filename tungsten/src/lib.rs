#![crate_name = "tungsten"]
#![crate_type = "lib"]
#![allow(dead_code)]

//extern crate cgmath;
#[macro_use]
extern crate log;
//general time
extern crate time;

mod kernal;

use std::default::Default;
use std::marker::PhantomData;

//Engine version variables in sync with the one in cargo
const VERSION_MAJOR: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &'static str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &'static str = env!("CARGO_PKG_VERSION_PATCH");

///The Engine the basic struct from which all other data structs spawn
pub struct Engine<G,Win,Ren,Con,Phy,Set,Res>
    where G: Game,
          Win: System,
          Ren: System,
          Con: System,
          Phy: System,
          Set: System{

    pg: PhantomData<G>,
    pw: PhantomData<Win>,
    pr: PhantomData<Ren>,
    pc: PhantomData<Con>,
    pp: PhantomData<Phy>,
    ps: PhantomData<Set>,
    ps: PhantomData<Res>,
}

type TungWin = ();
type TungRen = ();
type TungRes = ();
type TungCon = ();
type TungPhy = ();
type TungSet = ();
type TungRes = ();

type TungEngine<G> = Engine<G,TungWin,TungRen,TungCon,TungPhy,TungSet,TungRes>;

enum Error{
    Null,
}

impl<G,Win,Ren,Con,Phy,Set,Res> Engine<G,Win,Ren,Con,Phy,Set,Res>
    where G: Game,
          Win: System,
          Ren: System,
          Con: System,
          Phy: System,
          Set: System{

    pub fn Go(){
        let root = Root::<G::GameData
                ,Win::SystemData
                ,Ren::SystemData
                ,Con::SystemData
                ,Phy::SystemData
                ,Set::SystemData>::new();
    }
}


///The root data structure.
///All systems which need to hold onto data between should place it here.
pub struct Root<GD,WD,RD,CD,PD,SD,RED>
    where GD: Default,
          WD: Default,
          RD: Default,
          CD: Default,
          PD: Default,
          SD: Default,
          RED: Default,{
    game: GD,
    console: CD,
    window: WD,
    render: RD,
    physics: PD,
    resource: RED,
    settings: SD,
}

impl<GD,WD,RD,CD,PD,SD,RED> Root<GD,WD,RD,CD,PD,SD,RED>
    where GD: Default,
          WD: Default,
          RD: Default,
          CD: Default,
          PD: Default,
          SD: Default,
          RED: Default,{
    fn new() -> Self{
        Default::default();
    }
}

///The trait used when implementing your own game
pub trait Game{
    type GameData: Default;
    type EventType;

    fn new() -> Self;
    fn update();
    fn render();
    fn init();
}

//The Event 
pub enum Event{
    Input(()),
}

trait System{
    type SystemData: Default;

    fn new(data: &mut Self::SystemData) -> Self;

    fn run(&mut self){
    }
}

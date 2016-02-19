//!
//! The Kernal
//!
//! The heart of the game engine.
//! This is where the blood of cpu time is pumped through the engine.
//! This is where the systems live.
//!

extern crate num_cpus;

use super::Root;
use super::Game;

use super::schedular::Schedular;


pub trait System{
    fn run(&mut self,schedular: &mut Schedular);
}


pub struct Kernal<'a,G: Game + 'a>{
    root: &'a mut Root<G>,
    cpus: usize,
    systems: Vec<Box<System>>,
    schedular: Schedular,
    running: bool,
}

impl<'a,G: Game + 'a> Kernal<'a,G>{
    pub fn new(root: &'a mut Root<G>) -> Self{
        info!("Kernal Created.");
        let num_cpus = num_cpus::get();
        info!("Found {} cores.",num_cpus);
        Kernal{
            root: root,
            cpus: num_cpus,
            systems: Vec::new(),
            schedular: Schedular::new(num_cpus),
            running: true,
        }
    }

    pub fn add_system(&mut self,sys: Box<System>){
        self.systems.push(sys);
        self.systems.shrink_to_fit();
    }

    pub fn run(&mut self){
        while self.running {
            for sys in &mut self.systems{
                sys.run(&mut self.schedular);
            }
        }
    }

    pub fn quit(&mut self){
        self.running = false;
    }
}

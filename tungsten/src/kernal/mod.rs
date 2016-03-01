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

mod schedular;
pub use self::schedular::Schedular;

mod thread_manager;
use self::thread_manager::ThreadManager;

pub trait System{
    fn run(&mut self,schedular: &mut Schedular);
}


pub struct Kernal<'a,G: Game + 'a>{
    root: &'a Root<G>,
    cpus: usize,
    systems: Vec<Box<System>>,
    thread_manager: ThreadManager,
    running: bool,
}

impl<'a,G: Game + 'a> Kernal<'a,G>{
    pub fn new(root: &'a Root<G>) -> Self{
        info!("Kernal Created.");
        let num_cpus = num_cpus::get();
        info!("Found {} cores.",num_cpus);
        Kernal{
            root: root,
            cpus: num_cpus,
            systems: Vec::new(),
            thread_manager: ThreadManager::new(),
            running: true,
        }
    }

    pub fn add_system(&mut self,sys: Box<System>){
        self.systems.push(sys);
    }

    pub fn run(&mut self){
        self.systems.shrink_to_fit();
        self.thread_manager.create(self.cpus);
        while self.running {
            println!("Systems");
            self.running = false;
            for sys in &mut self.systems{
                let mut schedular = Schedular::new();
                sys.run(&mut schedular);
                schedular.flush(&mut self.thread_manager);
            }
        }
    }

    pub fn quit(&mut self){
        self.running = false;
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use super::schedular::Job;
    use super::schedular::JobError;
    use super::super::game::Game;
    use super::super::Root;

    struct HelloWorld;
    struct HelloJob{
        test: u64,
    }
    struct HelloGame;

    impl Job for HelloJob{
        fn execute(&mut self) -> Result<(),JobError>{
            println!("Hello world: {}",self.test);
            Ok(())
        }
    }

    impl System for HelloWorld{
        fn run(&mut self,sched: &mut Schedular){
            for i in 0..10{
                sched.add_job(Box::new(HelloJob{
                    test:i,
                }));
            }
        }
    }
    
    impl Game for HelloGame{
        fn new() -> Self{
            HelloGame
        }
    }

    #[test]
    fn kernal(){
        let root = Root::<HelloGame>::new();
        let mut kernal = Kernal::new(&root);
        kernal.add_system(Box::<HelloWorld>::new(HelloWorld));
        println!("Running");
        kernal.run();
    }
}

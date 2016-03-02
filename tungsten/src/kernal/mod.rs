//!
//! The Kernal
//!
//! The heart of the game engine.
//! This is where the blood of cpu time is pumped through the engine.
//! This is where the systems live.
//!


use super::Root;
use super::Game;

use std::sync::atomic::Ordering;

mod schedular;
pub use self::schedular::Schedular;

mod thread_manager;
use self::thread_manager::ThreadManager;


pub trait System{
    fn run(&mut self,schedular: &mut Schedular);
}


pub struct Kernal<'a,G: Game + 'a>{
    root: &'a Root<G>,
    systems: Vec<Box<System>>,
    thread_manager: ThreadManager,
}

impl<'a,G: Game + 'a> Kernal<'a,G>{
    pub fn new(root: &'a Root<G>) -> Self{
        info!("Kernal Created.");
        Kernal{
            root: root,
            systems: Vec::new(),
            thread_manager: ThreadManager::new(),
        }
    }

    pub fn add_system(&mut self,sys: Box<System>){
        self.systems.push(sys);
    }

    pub fn run(&mut self){
        self.systems.shrink_to_fit();
        self.thread_manager.create(self.root.platform.cores);
        //Game loop
        while self.root.running.load(Ordering::Relaxed){
            for sys in &mut self.systems{
                let mut schedular = Schedular::new();
                sys.run(&mut schedular);
                schedular.flush(&mut self.thread_manager);
            }
            trace!("Frame end");
        }
        //end loop
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
    fn kernal_hello(){
        let root = Root::<HelloGame>::new();
        let mut kernal = Kernal::new(&root);
        kernal.add_system(Box::<HelloWorld>::new(HelloWorld));
        println!("Running");
        kernal.run();
    }

    fn fibbo(num: u64) -> u64{
        match num{
            0 => 1,
            1 => 1,
            x => fibbo(x-1) + fibbo(x -2),
        }
        /*
        if num == 0 || num == 1{
            1
        }else{
            let mut first = 1;
            let mut second = 1;
            for i in 2..num+1{
                let new = first + second;
                first = second;
                second = new;
            }
            second
        }
        */
    }

    struct FibboWorld;
    struct FibboJob{
        test: u64,
    }
    impl Job for FibboJob{
        fn execute(&mut self) -> Result<(),JobError>{
            println!("fibbo,{} = {}",self.test,fibbo(self.test));
            Ok(())
        }
    }

    impl System for FibboWorld{
        fn run(&mut self,sched: &mut Schedular){
            for i in 20..44{
                sched.add_job(Box::new(FibboJob{
                    test:i,
                }));
            }
        }
    }

    #[test]
    fn kernal_work(){
        let root = Root::<HelloGame>::new();
        let mut kernal = Kernal::new(&root);
        kernal.add_system(Box::new(FibboWorld));
        println!("Running");
        kernal.run();
    }
}

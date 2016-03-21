
use super::Root;

mod schedular;
pub use self::schedular::Schedular;

mod job_manager;
pub use self::job_manager::Job;
pub use self::job_manager::JobError;
use self::job_manager::JobManager;

mod thread_manager;

/// A trait for object which can create jobs.
pub trait System{
    /// A function called when the system needs to run.
    fn run(&mut self,root: &Root,schedular: &mut Schedular);
}

/// The heart of the engine, the kernal keeps the engine running 
/// and manages all the jobs.
pub struct Kernal<'a>{
    root: &'a Root,
    systems: Vec<Box<System>>,
    job_manager: JobManager,
}

impl<'a> Kernal<'a>{
    pub fn new(root: &'a Root) -> Self{
        info!("Kernal Created.");
        Kernal{
            job_manager: JobManager::new(root.async.platform.cores),
            root: root,
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self,sys: Box<System>){
        self.systems.push(sys);
    }

    pub fn run(&mut self){
        self.systems.shrink_to_fit();
        //Game loop
        while self.root.async.running.should(){
            for sys in &mut self.systems{
                let mut schedular = Schedular::new();
                sys.run(self.root,&mut schedular);
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
    use std::sync::atomic::Ordering;

    struct HelloWorld;
    struct HelloJob{
        test: u64,
    }
    struct HelloGame;
    
    impl Game for HelloGame{}

    impl Job for HelloJob{
        fn execute(&mut self) -> Result<(),JobError>{
            println!("Hello world: {}",self.test);
            Ok(())
        }
    }

    impl System for HelloWorld{
        fn run(&mut self,root: &Root,sched: &mut Schedular){
            for i in 0..10{
                sched.add_job(Box::new(HelloJob{
                    test:i,
                }));
            }
            root.running.quit();
        }
    }
    
    #[test]
    fn kernal_hello(){
        let root = Root::new(HelloGame);
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
        fn run(&mut self,root: &Root,sched: &mut Schedular){
            for i in 20..44{
                sched.add_job(Box::new(FibboJob{
                    test:i,
                }));
            }
            root.running.quit();
        }
    }

    #[test]
    fn kernal_work(){
        let root = Root::new(HelloGame);
        let mut kernal = Kernal::new(&root);
        kernal.add_system(Box::new(FibboWorld));
        println!("Running");
        kernal.run();
    }
}

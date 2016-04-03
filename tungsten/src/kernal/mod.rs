
use super::Root;

mod schedular;
pub use self::schedular::Schedular;

mod job_manager;
pub use self::job_manager::Job;
pub use self::job_manager::JobError;
pub use self::job_manager::JobBuilder;
use self::job_manager::JobManager;

mod thread_manager;

/// A trait for object which can create jobs.
pub trait System{
    /// A function called when the system needs to run.
    fn run(&mut self,root: &Root) -> Option<JobBuilder>;
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
            job_manager: JobManager::new(root.async.platform.cores,root),
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
                if let Some(builder) = sys.run(self.root){
                    self.job_manager.add_jobs(builder);
                }
                self.job_manager.update();
            }
            self.job_manager.frame();
            trace!("Frame end");
        }
        //end loop
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use super::super::game::Game;
    use super::super::Root;

    use super::super::AsyncRoot;

    use std::sync::Arc;
    //use std::cell::UnsafeCell;
    use std::sync::atomic::AtomicUsize;
    //use std::sync::atomic::Ordering;

    struct HelloWorld;
    struct HelloJob{
        test: u64,
    }
    struct HelloGame;
    
    impl Game for HelloGame{}

    impl Job for HelloJob{
        fn execute(&mut self,_:&AsyncRoot) -> Result<(),JobError>{
            println!("Hello world: {}",self.test);
            Ok(())
        }
    }

    impl System for HelloWorld{
        fn run(&mut self,root: &Root) -> Option<JobBuilder>{
            let mut job_builder = JobBuilder::new();
            for i in 0..10{
                job_builder.add_job(Box::new(HelloJob{
                    test:i,
                }));
            }
            root.async.running.quit();
            Some(job_builder)
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

    struct HelloWorldSync{
        result: Arc<(AtomicUsize,[usize; 100])>,
    }

    struct HelloJobSync{
        result: Arc<(AtomicUsize,[usize; 100])>,
    }
    /*

    impl Job for HelloJobSync{
        fn execute(&mut self,_: &AsyncRoot) -> Result<(),JobError>{
            let index = self.result.0.fetch_add(1,Ordering::AcqRel);
            Ok(())
        }
    }

    impl System for HelloWorldSync{
        fn run(&mut self,root: &Root) -> Option<JobBuilder>{
            let mut job_builder = JobBuilder::new();
            job_builder.add_job(Box::new(HelloJobSync{
                result: self.result.clone(),
            }));
            for i in 1..100{
                job_builder.add_fence();
                job_builder.add_job(Box::new(HelloJobSync{
                    result: self.result.clone(),
                }));
            }
            root.async.running.quit();
            Some(job_builder)
        }
    }

    #[test]
    fn kernal_hello_syncronisation(){
        let result = Arc::new((AtomicUsize::new(0),[0;100]));
        let sys = HelloJobSync{
            result: result.clone(),
        };
    }
    */

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
        fn execute(&mut self,_:&AsyncRoot) -> Result<(),JobError>{
            println!("fibbo,{} = {}",self.test,fibbo(self.test));
            Ok(())
        }
    }

    impl System for FibboWorld{
        fn run(&mut self,root: &Root)-> Option<JobBuilder>{
            let mut job_builder = JobBuilder::new();
            for i in 20..44{
                job_builder.add_job(Box::new(FibboJob{
                    test:i,
                }));
            }
            root.async.running.quit();
            Some(job_builder)
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

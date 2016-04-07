
use super::Root;

mod task_manager;
pub use self::task_manager::Task;
pub use self::task_manager::TaskError;
pub use self::task_manager::TaskBuilder;
use self::task_manager::TaskManager;

mod thread_manager;

/// A trait for object which can create tasks.
pub trait System{
    /// A function called when the system needs to run.
    fn run(&mut self,root: &Root) -> Option<TaskBuilder>;
}

/// The heart of the engine, the kernel keeps the engine running 
/// and manages all the tasks.
pub struct Kernel<'a>{
    root: &'a Root,
    systems: Vec<Box<System>>,
    task_manager: TaskManager,
}

impl<'a> Kernel<'a>{
    pub fn new(root: &'a Root) -> Self{
        info!("Kernel Created.");
        Kernel{
            task_manager: TaskManager::new(root.async.platform.cores,root),
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
                    self.task_manager.add_tasks(builder);
                }
                self.task_manager.update();
            }
            self.task_manager.frame();
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
    struct HelloTask{
        test: u64,
    }
    struct HelloGame;
    
    impl Game for HelloGame{}

    impl Task for HelloTask{
        fn execute(&mut self,_:&AsyncRoot) -> Result<(),TaskError>{
            println!("Hello world: {}",self.test);
            Ok(())
        }
    }

    impl System for HelloWorld{
        fn run(&mut self,root: &Root) -> Option<TaskBuilder>{
            let mut task_builder = TaskBuilder::new();
            for i in 0..10{
                task_builder.add_task(Box::new(HelloTask{
                    test:i,
                }));
            }
            root.async.running.quit();
            Some(task_builder)
        }
    }
    
    #[test]
    fn kernel_hello(){
        let root = Root::new(HelloGame);
        let mut kernel = Kernel::new(&root);
        kernel.add_system(Box::<HelloWorld>::new(HelloWorld));
        println!("Running");
        kernel.run();
    }

    struct HelloWorldSync{
        result: Arc<(AtomicUsize,[usize; 100])>,
    }

    struct HelloTaskSync{
        result: Arc<(AtomicUsize,[usize; 100])>,
    }
    /*

    impl Task for HelloTaskSync{
        fn execute(&mut self,_: &AsyncRoot) -> Result<(),TaskError>{
            let index = self.result.0.fetch_add(1,Ordering::AcqRel);
            Ok(())
        }
    }

    impl System for HelloWorldSync{
        fn run(&mut self,root: &Root) -> Option<TaskBuilder>{
            let mut task_builder = TaskBuilder::new();
            task_builder.add_task(Box::new(HelloTaskSync{
                result: self.result.clone(),
            }));
            for i in 1..100{
                task_builder.add_fence();
                task_builder.add_task(Box::new(HelloTaskSync{
                    result: self.result.clone(),
                }));
            }
            root.async.running.quit();
            Some(task_builder)
        }
    }

    #[test]
    fn kernel_hello_syncronisation(){
        let result = Arc::new((AtomicUsize::new(0),[0;100]));
        let sys = HelloTaskSync{
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
    struct FibboTask{
        test: u64,
    }
    impl Task for FibboTask{
        fn execute(&mut self,_:&AsyncRoot) -> Result<(),TaskError>{
            println!("fibbo,{} = {}",self.test,fibbo(self.test));
            Ok(())
        }
    }

    impl System for FibboWorld{
        fn run(&mut self,root: &Root)-> Option<TaskBuilder>{
            let mut task_builder = TaskBuilder::new();
            for i in 20..44{
                task_builder.add_task(Box::new(FibboTask{
                    test:i,
                }));
            }
            root.async.running.quit();
            Some(task_builder)
        }
    }

    #[test]
    fn kernel_work(){
        let root = Root::new(HelloGame);
        let mut kernel = Kernel::new(&root);
        kernel.add_system(Box::new(FibboWorld));
        println!("Running");
        kernel.run();
    }
}

use std::thread::JoinHandle;
use std::thread::Builder;

use std::thread;

pub struct ThreadManager{
    joins: Vec<JoinHandle<()>>,
}

impl ThreadManager{
    pub fn new(amount: usize) -> Self{
        let mut vec = Vec::new();
        for _ in 0..amount{
            vec.push(
                Builder::new()
                    .name("Tungsten Worker Thread".to_string())
                    .spawn(||{
                        thread::park();
                        info!("Worker thread quit");
                    })
                    .expect("Could not create thread"));
        }
        ThreadManager{
            joins: vec,
        }
    }
}

impl Drop for ThreadManager{
    fn drop(&mut self){
        for t in &self.joins{
            t.thread().unpark();
        }
        for t in self.joins.drain(..){
            t.join().expect("Error while joining thread");
        }
    }
}

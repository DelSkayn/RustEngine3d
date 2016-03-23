use std::thread::JoinHandle;
use std::thread;

pub struct ThreadManager{
    threads: Vec<JoinHandle<()>>,
}

impl ThreadManager{
    pub fn new() -> Self{
        ThreadManager{
            threads: Vec::new(),
        }
    }

    pub fn add_thread<F: Fn()+Send+'static>(&mut self,func: F){
        self.threads.push(thread::spawn(func));
    }

    pub fn amount(&self) -> usize{
        self.threads.len()
    }
}

impl Drop for ThreadManager{
    fn drop(&mut self){
        for t in self.threads.drain(..){
            t.join().expect("Error while joining thread");
        }
    }
}

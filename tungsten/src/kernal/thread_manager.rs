use std::thread::JoinHandle;
use std::thread::Builder;
use std::thread;

use std::sync::mpsc::*;
use std::sync::atomic::AtomicBool;

use super::schedular::Job;

use std::sync::Arc;

use super::super::crossbeam::sync::TreiberStack;

struct Thread{
    job_queue: *const TreiberStack<Box<Job>>,
}

impl Thread{
    fn run(&mut self){
        thread::park();
    }
}

struct ThreadData{
    join: JoinHandle<()>,
}

pub struct ThreadManager{
    threads: Vec<ThreadData>,
    job_queue: TreiberStack<Box<Job>>,
}

impl ThreadManager{
    pub fn new() -> Self{
        ThreadManager{
            threads: Vec::new(),
            job_queue: TreiberStack::new(),
        }
    }

    pub fn create(&mut self,amount: usize){
        let raw = (&self.job_queue) as *const TreiberStack<Box<Job>>;
        for _ in 0..amount{
            self.threads.push(
                ThreadData{
                    join: Builder::new()
                        .name("Tungsten Worker Thread".to_string())
                        .spawn(move||{
                            let mut thread;
                            unsafe{
                                thread = Thread{
                                    job_queue: raw,
                                };
                            }
                            thread.run();
                        })
                    .expect("Could not create thread"),
                });
        }
    }
}

impl Drop for ThreadManager{
    fn drop(&mut self){
        for t in &self.threads{
            t.join.thread().unpark();
        }
        for t in self.threads.drain(..){
            t.join.join().expect("Error while joining thread");
        }
    }
}

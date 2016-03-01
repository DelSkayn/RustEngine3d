use std::thread::JoinHandle;
use std::thread;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use super::schedular::Job;
use super::schedular::JobError;

use crossbeam::sync::MsQueue;


struct NullJob;

impl Job for NullJob{
    fn execute(&mut self) -> Result<(),JobError>{
        Ok(())
    }
}

struct Thread{
    id: u8,
    running: Arc<AtomicBool>,
    job_queue: Arc<MsQueue<Box<Job>>>,
}

impl Thread{
    fn run(&mut self){
        println!("Thread running");
        while self.running.load(Ordering::Relaxed){
            let mut x = self.job_queue.pop();
            x.execute().unwrap();
        }
    }
}

struct ThreadData{
    join: JoinHandle<()>,
}

pub struct ThreadManager{
    running: Arc<AtomicBool>,
    threads: Vec<ThreadData>,
    pub job_queue: Arc<MsQueue<Box<Job>>>,
}

impl ThreadManager{
    pub fn new() -> Self{
        ThreadManager{
            running: Arc::new(AtomicBool::new(true)),
            threads: Vec::new(),
            job_queue: Arc::new(MsQueue::new()),
        }
    }

    pub fn create<'a>(&'a mut self,amount: usize){
        for i in 0..amount{
            let running = self.running.clone();
            let job_que = self.job_queue.clone();
            let data  = ThreadData{
                join: thread::spawn(move ||{
                    let mut thread;
                    thread = Thread{
                        id: i as u8,
                        running: running,
                        job_queue: job_que,
                    };
                    thread.run();
                }),
            };
            self.threads.push(data);
        }
    }

    pub fn add_job(&self, job: Box<Job>){
        println!("Job added");
        self.job_queue.push(job);
    }

    pub fn wake(&self){
        for _ in &self.threads{
        }
    }
}

impl Drop for ThreadManager{
    fn drop(&mut self){
        self.running.store(false,Ordering::Relaxed);
        for _ in 0..self.threads.len(){
            self.job_queue.push(Box::new(NullJob));
        }
        for t in &self.threads{
            t.join.thread().unpark();
        }
        for t in self.threads.drain(..){
            t.join.join().expect("Error while joining thread");
        }
    }
}

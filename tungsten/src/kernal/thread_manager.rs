use std::thread::JoinHandle;
use std::thread;

use std::sync::Arc;

use super::schedular::Job;
use super::schedular::JobError;

use crossbeam::sync::MsQueue;


struct QuitJob;


//TODO switch from result to internal format.
impl Job for QuitJob{
    fn execute(&mut self) -> Result<(),JobError>{
        Err(JobError::Quiting)
    }
}

struct Thread{
    id: u8,
    //might need to switch this to a work steal
    //cycle if performance is bad.
    //Dont think it will be much of an isue
    job_queue: Arc<MsQueue<Box<Job>>>,
}

impl Thread{
    fn run(&mut self){
        'main: loop{
            let mut job = self.job_queue.pop();
            //FIXME handle more errors
            match job.execute(){
                Err(JobError::Quiting) => {break 'main;},
                _ => {unimplemented!()},
            }
        }
    }
}

struct ThreadData{
    join: JoinHandle<()>,
}

pub struct ThreadManager{
    threads: Vec<ThreadData>,
    pub job_queue: Arc<MsQueue<Box<Job>>>,
    pub notify: Arc<()>,
}

impl ThreadManager{
    pub fn new() -> Self{
        ThreadManager{
            threads: Vec::new(),
            job_queue: Arc::new(MsQueue::new()),
            notify: Arc::new(),
        }
    }

    pub fn create<'a>(&'a mut self,amount: usize){
        for i in 0..amount{
            let job_que = self.job_queue.clone();
            let data  = ThreadData{
                join: thread::spawn(move ||{
                    let mut thread;
                    thread = Thread{
                        id: i as u8,
                        job_queue: job_que,
                    };
                    thread.run();
                }),
            };
            self.threads.push(data);
        }
    }

    pub fn add_job(&self, job: Box<Job>){
        self.job_queue.push(job);
    }
}

impl Drop for ThreadManager{
    fn drop(&mut self){
        for _ in 0..self.threads.len(){
            self.job_queue.push(Box::new(QuitJob));
        }
        for t in self.threads.drain(..){
            t.join.join().expect("Error while joining thread");
        }
    }
}

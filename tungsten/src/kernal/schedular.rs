use super::thread_manager::ThreadManager;

#[derive(Debug)]
pub enum JobError{
    Failed(&'static str)
}

#[derive(PartialEq)]
pub struct JobId(u64);

impl JobId{
    pub fn from_u64(id: u64) -> Self{
        JobId(id)
    }
}

pub trait Job: Send + Sync{
    fn execute(&mut self) -> Result<(),JobError>;

    fn after(&self) -> Option<JobId>{
        None
    }

    fn next(&mut self) -> Option<Box<Job>>{
        None
    }
}

struct NullJob;

impl Job for NullJob{
    fn execute(&mut self) -> Result<(), JobError>{
        Ok(())
    }
}


pub struct Schedular{
    jobs: Vec<Box<Job>>,
}

impl Schedular{
    pub fn new() -> Self{
        Schedular{
            jobs: Vec::new(),
        }
    }

    pub fn add_job(&mut self,job: Box<Job>){
        self.jobs.push(job);
    }

    pub fn flush(&mut self,threads: &mut ThreadManager){
        for j in self.jobs.drain(..){
            threads.add_job(j);
        }
        threads.wake();
    }
}

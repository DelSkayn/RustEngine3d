use super::thread_manager::ThreadManager;

#[derive(Debug)]
pub enum JobError{
    Failed(&'static str),
    Quiting,
}

#[derive(PartialEq)]
pub struct JobId(u64);

impl JobId{
    pub fn from_u64(id: u64) -> Self{
        JobId(id)
    }
}

/// A trait for objects which can be executed as 
/// jobs on threads.
pub trait Job: Send + Sync{
    /// Executed when the job needs to be run.
    /// Can fail.
    fn execute(&mut self) -> Result<(),JobError>;

    /// Retruns if there needs to be a job executed after this one is 
    /// finished.
    /// WARN: Possible removed.
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

/// Struct used for scheduling jobs.
pub struct Schedular{
    jobs: Vec<Box<Job>>,
}

impl Schedular{
    /// returns a new schedular.
    pub fn new() -> Self{
        Schedular{
            jobs: Vec::new(),
        }
    }

    /// Adds a job to be scheduled.
    pub fn add_job(&mut self,job: Box<Job>){
        self.jobs.push(job);
    }

    /// Flushes all the jobs to a ThreadManager.
    /// HIGHLY TEMP.
    /// Not stable interface.
    pub fn flush(&mut self,threads: &mut ThreadManager){
        for j in self.jobs.drain(..){
            threads.add_job(j);
        }
    }
}

use super::job_manager::Job;

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
}

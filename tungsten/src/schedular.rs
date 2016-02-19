
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

pub trait Job{
    fn execute(&mut self) -> Result<(),JobError>;

    fn after(&self) -> Option<JobId>{
        None
    }

    fn next(&mut self) -> Option<Box<Job>>{
        None
    }
}

pub struct Schedular{
    jobs: Vec<Box<Job>>,
}

impl Schedular{
    pub fn new(num_cpus: usize) -> Self{
        Schedular{
            jobs: Vec::new(),
        }
    }

    pub fn add_job(&mut self,_job: Box<Job>){
        unimplemented!();
    }
}

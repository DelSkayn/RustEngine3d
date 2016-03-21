use super::thread_manager::ThreadManager;

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use std::collections::VecDeque;

use std::thread;

use std::time::Duration;

use crossbeam::sync::MsQueue;


#[derive(Debug)]
pub enum JobError{
    Failed(&'static str),
    Retry,
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

struct JobFramePart{
    //the jobs to add WHEN DONE IS 0
    jobs: Vec<SendJobStruct>,
    //when 0 the above jobs are added.
    done: Arc<AtomicUsize>,
}

struct JobQueue{
    job_parts: Vec<VecDeque<JobFramePart>>,
}

impl JobQueue{
    fn new() -> Self{
        JobQueue{
            job_parts: Vec::new(),
        }
    }

    pub fn add_jobs(&mut self,jobs: VecDeque<JobFramePart>){
        self.job_parts.push(jobs);
    }

    pub fn get_active(&mut self) -> Vec<SendJobStruct>{
        let mut res = Vec::new();
        //uuhg slowww reallocation
        for ref mut job_part in (&mut self.job_parts).into_iter(){
            if job_part.front().unwrap().done.load(Ordering::Acquire) == 0{
                let mut next = job_part.pop_front().unwrap();
                res.append(&mut next.jobs);
            }
        }
        self.job_parts.retain(|job_part|{
            !job_part.is_empty()
        });
        res
    }

    pub fn done(&self) -> bool{
        self.job_parts.is_empty()
    }
}
 
struct SendJobStruct{
    done: Option<Arc<AtomicUsize>>,
    job: Box<Job>,
}

pub struct JobBuilder{
    jobs: VecDeque<JobFramePart>,
    start_jobs: Vec<SendJobStruct>,
    after_fence: bool,
    index: usize,
}

impl JobBuilder{

    fn new() -> JobBuilder{
        JobBuilder{
            jobs: VecDeque::new(),
            start_jobs: Vec::new(),
            after_fence: false,
            index: 0,
        }
    }

    fn add_job(&mut self,job: Box<Job>){
        if self.after_fence{
            let count = self.jobs[self.index+1].done.clone();
            self.jobs[self.index].jobs.push(SendJobStruct{
                done: Some(count),
                job: job,
            })
        }else{
            self.start_jobs.push(
                SendJobStruct{
                    job: job,
                    done: None,
                });
        }
    }
    fn add_fence(&mut self){
        if !self.after_fence{
            self.after_fence = true;
            let arc = Arc::new(AtomicUsize::new(0));
            for ref mut job in &mut self.start_jobs{
                job.done = Some(arc.clone());
            }
            self.jobs.push_back(JobFramePart{
                jobs: Vec::new(),
                done: arc,
            });
            self.jobs.push_back(JobFramePart{
                jobs: Vec::new(),
                done: Arc::new(AtomicUsize::new(0)),
            });
        }else{
        }
    }
}

pub struct JobManager{
    threads: ThreadManager,
    active_job_queue: Arc<MsQueue<SendJobStruct>>,
    pending_job_queue: JobQueue,
}

impl JobManager{
    pub fn new(amount: usize) -> Self{
        let job_que = Arc::new(MsQueue::new());
        let mut threads = ThreadManager::new();
        let thread = thread::current();

        for _ in 0..amount{
            let queue = job_que.clone();
            let main_thread = thread.clone();
            threads.add_thread(move ||{
                loop{
                    let mut job: SendJobStruct = queue.pop();
                    job.job.execute().unwrap();
                    if let Some(x) = job.done{
                        if x.fetch_sub(1,Ordering::Release) == 0{
                            main_thread.unpark();
                        }
                    }
                }
            });
        }

        JobManager{
            threads: threads,
            active_job_queue: job_que,
            pending_job_queue: JobQueue::new(),
        }
    }

    pub fn update(&mut self){
        let new = self.pending_job_queue.get_active();
        for job in new{
            self.active_job_queue.push(job);
        }
    }

    /// Waits until the current frame is done.
    /// Should be called each time after an entire frame is done.
    pub fn frame(&mut self){
        while self.pending_job_queue.done(){
            self.update();
            thread::park_timeout(Duration::from_millis(1));
        }
    }
}

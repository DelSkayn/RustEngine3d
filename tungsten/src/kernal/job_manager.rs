use super::thread_manager::ThreadManager;

use super::super::Root;
use super::super::AsyncRoot;

use super::super::util::Running;

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
    fn execute(&mut self,root: &AsyncRoot) -> Result<(),JobError>;

    /// Retruns if there needs to be a job executed after this one is 
    /// finished.
    /// WARN: Possible removed.
    fn next(&mut self) -> Option<Box<Job>>{
        // I think this will still be nessary maybe in a different form.
        None
    }
}

struct NullJob;

impl Job for NullJob{
    fn execute(&mut self,_: &AsyncRoot) -> Result<(), JobError>{
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


    //Prob better to directly push to the queue.
    pub fn get_active(&mut self) -> Vec<SendJobStruct>{
        let mut res = Vec::new();
        //uuhg slowww reallocation
        for ref mut job_part in (&mut self.job_parts).into_iter(){
            if !job_part.is_empty(){
                if job_part.front().unwrap().done.load(Ordering::Acquire) == 0{
                    let mut next = job_part.pop_front().unwrap();
                    res.append(&mut next.jobs);
                }
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

    pub fn new() -> JobBuilder{
        JobBuilder{
            jobs: VecDeque::new(),
            start_jobs: Vec::new(),
            after_fence: false,
            index: 0,
        }
    }

    pub fn add_job(&mut self,job: Box<Job>){
        if self.after_fence{
            let count = self.jobs[self.index+1].done.clone();
            count.fetch_add(1,Ordering::Relaxed);
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
    pub fn add_fence(&mut self){
        if !self.after_fence{
            self.after_fence = true;
            let arc = Arc::new(AtomicUsize::new(0));
            for ref mut job in &mut self.start_jobs{
                job.done = Some(arc.clone());
            }
            arc.store(self.start_jobs.len(),Ordering::Relaxed);
            self.jobs.push_back(JobFramePart{
                jobs: Vec::new(),
                done: arc,
            });
            self.jobs.push_back(JobFramePart{
                jobs: Vec::new(),
                done: Arc::new(AtomicUsize::new(0)),
            });
        }else{
            self.index += 1;
            self.jobs.push_back(JobFramePart{
                jobs: Vec::new(),
                done: Arc::new(AtomicUsize::new(0)),
            });
        }
    }
}

pub struct JobManager{
    threads: ThreadManager,
    active_job_queue: Arc<MsQueue<SendJobStruct>>,
    running: Arc<Running>,
    pending_job_queue: JobQueue,
}

impl JobManager{
    pub fn new(amount: usize,root: &Root) -> Self{
        let job_que = Arc::new(MsQueue::new());
        let mut threads = ThreadManager::new();
        let thread = thread::current();
        let running = Arc::new(Running::new());

        let amount = if amount > 1 {
            amount -1
        }else{
            amount
        };

        for _ in 0..amount{
            let queue = job_que.clone();
            let main_thread = thread.clone();
            let run = running.clone();
            let aroot = root.async.clone();
            threads.add_thread(move ||{
                while run.should(){
                    let mut job: SendJobStruct = queue.pop();
                    match job.job.execute(&aroot){
                        Ok(_) => {},
                        _ => unimplemented!(),
                    }
                    if let Some(x) = job.done{
                        if x.fetch_sub(1,Ordering::Release) == 1{
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
            running: running,
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
        while !self.pending_job_queue.done(){
            println!("Updating end of frame");
            self.update();
            thread::park_timeout(Duration::from_millis(100));
        }
    }

    pub fn add_jobs(&mut self,jobs: JobBuilder){
        for job in jobs.start_jobs{
            self.active_job_queue.push(job);
        }
        self.pending_job_queue.add_jobs(jobs.jobs);
    }
}

impl Drop for JobManager{
    fn drop(&mut self){
        self.running.quit();
        for _ in 0..self.threads.amount(){
            self.active_job_queue.push(SendJobStruct{
                job: Box::new(NullJob),
                done: None
            });
        }
    }
}

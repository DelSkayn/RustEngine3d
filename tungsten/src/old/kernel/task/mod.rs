
use std::sync::atomic::AtomicIsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use super::super::crossbeam;
use self::crossbeam::chase_lev;
use self::crossbeam::chase_lev::{
    Worker,
    Stealer,
};

mod task_thread;
use self::task_thread::TaskManger;

pub type TaskType = Fn<TaskManger>;

use super::thread_manager::ThreadManager;


fn init(th: &mut ThreadManager,amount_threads: usize) -> TaskWorker{
    let stealers = Vec::with_capacity(amount_threads);
    let workers = Vec::with_capacity(amount_threads);
    
    for _ in 0..amount_threads{
        let (w,s) = chase_lev::deque();
        stealers.push(s);
        workers.push(w);
    }

    let i = 0;
    for worker in 0..workers.drain(0..amount_threads-1){
        let stealer = stealers.clone();
        stealer.remove(i);
        th.spawn(||{
            TaskThread::run(worker,stealer);
        });
        i += 1;
    }

    stealers.remove(amount_threads-1);

    TaskWorker{
        stealer: stealer,
        worker: workers[0],
    }
}



struct TaskWorker{
    stealer: Vec<Stealer<Box<Task>>>,
    worker: Work<Box<Task>>,
}


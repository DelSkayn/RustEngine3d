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
pub enum TaskError{
    Failed(&'static str),
    Retry,
    Quiting,
}

#[derive(PartialEq)]
pub struct TaskId(u64);

impl TaskId{
    pub fn from_u64(id: u64) -> Self{
        TaskId(id)
    }
}

/// A trait for objects which can be executed as 
/// tasks on threads.
pub trait Task: Send + Sync{
    /// Executed when the task needs to be run.
    /// Can fail.
    fn execute(&mut self,root: &AsyncRoot) -> Result<(),TaskError>;

    /// Retruns if there needs to be a task executed after this one is 
    /// finished.
    /// WARN: Possible removed.
    fn next(&mut self) -> Option<Box<Task>>{
        // I think this will still be nessary maybe in a different form.
        None
    }
}

struct NullTask;

impl Task for NullTask{
    fn execute(&mut self,_: &AsyncRoot) -> Result<(), TaskError>{
        Ok(())
    }
}

struct TaskFramePart{
    //the tasks to add WHEN DONE IS 0
    tasks: Vec<SendTaskStruct>,
    //when 0 the above tasks are added.
    done: Arc<AtomicUsize>,
}

struct TaskQueue{
    task_parts: Vec<VecDeque<TaskFramePart>>,
}

impl TaskQueue{
    fn new() -> Self{
        TaskQueue{
            task_parts: Vec::new(),
        }
    }

    pub fn add_tasks(&mut self,tasks: VecDeque<TaskFramePart>){
        self.task_parts.push(tasks);
    }


    //Prob better to directly push to the queue.
    pub fn get_active(&mut self) -> Vec<SendTaskStruct>{
        let mut res = Vec::new();
        //uuhg slowww reallocation
        for ref mut task_part in (&mut self.task_parts).into_iter(){
            if !task_part.is_empty(){
                if task_part.front().unwrap().done.load(Ordering::Acquire) == 0{
                    let mut next = task_part.pop_front().unwrap();
                    res.append(&mut next.tasks);
                }
            }
        }
        self.task_parts.retain(|task_part|{
            !task_part.is_empty()
        });
        res
    }

    pub fn done(&self) -> bool{
        self.task_parts.is_empty()
    }
}
 
struct SendTaskStruct{
    done: Option<Arc<AtomicUsize>>,
    task: Box<Task>,
}

pub struct TaskBuilder{
    tasks: VecDeque<TaskFramePart>,
    start_tasks: Vec<SendTaskStruct>,
    after_fence: bool,
    index: usize,
}

impl TaskBuilder{

    pub fn new() -> TaskBuilder{
        TaskBuilder{
            tasks: VecDeque::new(),
            start_tasks: Vec::new(),
            after_fence: false,
            index: 0,
        }
    }

    pub fn add_task(&mut self,task: Box<Task>){
        if self.after_fence{
            let count = self.tasks[self.index+1].done.clone();
            count.fetch_add(1,Ordering::Relaxed);
            self.tasks[self.index].tasks.push(SendTaskStruct{
                done: Some(count),
                task: task,
            })
        }else{
            self.start_tasks.push(
                SendTaskStruct{
                    task: task,
                    done: None,
                });
        }
    }
    pub fn add_fence(&mut self){
        if !self.after_fence{
            self.after_fence = true;
            let arc = Arc::new(AtomicUsize::new(0));
            for ref mut task in &mut self.start_tasks{
                task.done = Some(arc.clone());
            }
            arc.store(self.start_tasks.len(),Ordering::Relaxed);
            self.tasks.push_back(TaskFramePart{
                tasks: Vec::new(),
                done: arc,
            });
            self.tasks.push_back(TaskFramePart{
                tasks: Vec::new(),
                done: Arc::new(AtomicUsize::new(0)),
            });
        }else{
            self.index += 1;
            self.tasks.push_back(TaskFramePart{
                tasks: Vec::new(),
                done: Arc::new(AtomicUsize::new(0)),
            });
        }
    }
}

pub struct TaskManager{
    threads: ThreadManager,
    active_task_queue: Arc<MsQueue<SendTaskStruct>>,
    running: Arc<Running>,
    pending_task_queue: TaskQueue,
}

impl TaskManager{
    pub fn new(amount: usize,root: &Root) -> Self{
        let task_que = Arc::new(MsQueue::new());
        let mut threads = ThreadManager::new();
        let thread = thread::current();
        let running = Arc::new(Running::new());

        let amount = if amount > 1 {
            amount -1
        }else{
            amount
        };

        for _ in 0..amount{
            let queue = task_que.clone();
            let main_thread = thread.clone();
            let run = running.clone();
            let aroot = root.async.clone();
            threads.add_thread(move ||{
                while run.should(){
                    let mut task: SendTaskStruct = queue.pop();
                    match task.task.execute(&aroot){
                        Ok(_) => {},
                        _ => unimplemented!(),
                    }
                    if let Some(x) = task.done{
                        if x.fetch_sub(1,Ordering::Release) == 1{
                            main_thread.unpark();
                        }
                    }
                }
            });
        }

        TaskManager{
            threads: threads,
            active_task_queue: task_que,
            pending_task_queue: TaskQueue::new(),
            running: running,
        }
    }

    pub fn update(&mut self){
        let new = self.pending_task_queue.get_active();
        for task in new{
            self.active_task_queue.push(task);
        }
    }

    /// Waits until the current frame is done.
    /// Should be called each time after an entire frame is done.
    pub fn frame(&mut self){
        while !self.pending_task_queue.done(){
            println!("Updating end of frame");
            self.update();
            thread::park_timeout(Duration::from_millis(100));
        }
    }

    pub fn add_tasks(&mut self,tasks: TaskBuilder){
        for task in tasks.start_tasks{
            self.active_task_queue.push(task);
        }
        self.pending_task_queue.add_tasks(tasks.tasks);
    }
}

impl Drop for TaskManager{
    fn drop(&mut self){
        self.running.quit();
        for _ in 0..self.threads.amount(){
            self.active_task_queue.push(SendTaskStruct{
                task: Box::new(NullTask),
                done: None
            });
        }
    }
}

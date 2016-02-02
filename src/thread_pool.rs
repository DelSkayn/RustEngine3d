extern crate num_cpus;
use std::thread::JoinHandle;
use std::thread::Builder;
use std::sync::mpsc::channel;
use std::sync::mpsc;

use std::cell::Cell;
use std::cell::RefCell;

use std::ops::Drop;

use std::collections::HashMap;

pub trait ThreadJob: Send{
    fn run(&mut self);
}

enum ThreadEvent{ 
    Job(u64,Box<ThreadJob>),
    Quit,
    Done(u64,Box<ThreadJob>),
}

pub struct JobId(u64);

impl JobId{
    fn to_int(&self) -> u64{
        self.0
    }
}


pub struct ThreadHandle{
    handle: JoinHandle<()>,//&'static Fn(&ThreadJob)>,
    sender: mpsc::Sender<ThreadEvent>,
    reciever: mpsc::Receiver<ThreadEvent>,
    current_jobs: u32,
}

pub struct ThreadManager{
    reciever: mpsc::Receiver<ThreadEvent>,
    sender: mpsc::Sender<ThreadEvent>,
    name: String,
}

impl ThreadManager{
    fn run(&mut self){
        debug!("{} started.",self.name);
        for e in &self.reciever{
            match e {
                ThreadEvent::Job(id,mut x) =>{
                    println!("{} executing job.",self.name);
                    x.run();
                    self.sender.send(ThreadEvent::Done(id,x)).unwrap();
                },
                ThreadEvent::Quit =>{
                    debug!("{} is stopped", self.name);
                    break;
                },
                _ => {panic!("Thread recieved unregonized event")},
            }
        }
    }
}

pub struct ThreadPool{
    threads: Vec<ThreadHandle>,
    num_threads: i32,
    next_thread: Cell<i32>,
    jobs: RefCell<HashMap<u64,Box<ThreadJob>>>,
    next_job_id: Cell<u64>,
}

impl ThreadPool{
    pub fn new() -> Self{
        let mut num = num_cpus::get() - 2;

        if num < 2{
            num = 2;
        }
        if num > 12{
            num = 12;
        }

        let mut threads = Vec::new();

        for i in 0..num{
            let (send_c,recv_s) = channel();
            let (send_s,recv_c) = channel();

            let name = format!("WorkerThread: {}",i);

            let manager = ThreadManager{
                reciever: recv_c,
                sender: send_c,
                name: name.clone(),
            };

            let thread = Builder::new()
                .name(name)
                .spawn(move || {
                    let mut man = manager;
                    man.run()
                }).unwrap();

            threads.push(ThreadHandle{
                    handle: thread,
                    sender: send_s,
                    reciever: recv_s,
                    current_jobs: 0,
                });

        }
        ThreadPool{
            threads: threads,
            num_threads: num as i32,
            next_thread: Cell::new(0),
            next_job_id: Cell::new(0),
            jobs: RefCell::new(HashMap::new()),
        }
    }

    pub fn run(&self,job: Box<ThreadJob>) -> JobId{
        let mut next = self.next_thread.get();
        next += 1;
        next %= self.num_threads;
        self.next_thread.set(next);
        let id = self.next_job_id.get();
        self.threads[next as usize].sender.send(ThreadEvent::Job(id,job)).unwrap();
        self.next_job_id.set(id+1);
        JobId(id)
    }

    pub fn get(&self,id: JobId) -> Option<Box<ThreadJob>>{
        self.jobs.borrow_mut().remove(&id.to_int())
    }
}


impl Drop for ThreadPool{
    fn drop(&mut self){
        for e in &self.threads{
            e.sender.send(ThreadEvent::Quit).unwrap();
        }
        while let Some(e) = self.threads.pop(){
            e.handle.join().unwrap();
        }
    }
}

mod test{
    use super::*;

    #[derive(Clone)]
    pub struct MessagePrinter{
        message: String,
    }

    impl ThreadJob for MessagePrinter{
        fn run(&mut self){
            println!("{}",self.message);
        }
    }

    #[test]
    fn test_thread_pool(){

        let tp = ThreadPool::new();

        let foo = Box::new(MessagePrinter{
            message: "printing!".to_string(),
        });
        let foo1 = Box::new(MessagePrinter{
            message: "form!".to_string(),
        });
        let foo2 = Box::new(MessagePrinter{
            message: "the!".to_string(),
        });
        let foo3 = Box::new(MessagePrinter{
            message: "threads!".to_string(),
        });

        tp.run(foo.clone());
        tp.run(foo1.clone());
        tp.run(foo2.clone());
        tp.run(foo3);
        tp.run(foo);
        tp.run(foo1);
        tp.run(foo2);
    }
}

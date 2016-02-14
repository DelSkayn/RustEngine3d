extern crate num_cpus;
use std::thread::JoinHandle;
use std::thread::Builder;
use std::sync::mpsc::channel;
use std::sync::mpsc;

use std::ops::Drop;

pub trait ThreadJob: Send{
    fn run(&mut self);
}

enum ThreadEvent{
    Job(Box<ThreadJob>),
    Quit,
    Done(Box<ThreadJob>),
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
                ThreadEvent::Job(mut x) =>{
                    x.run();
                    self.sender.send(ThreadEvent::Done(x)).unwrap();
                },
                ThreadEvent::Quit =>{
                    debug!("{} is stopped", self.name);
                    break;
                },
                _ => {},
            }
        }
    }
}


pub struct ThreadPool{
    threads: Vec<ThreadHandle>,
    job_id: u64,
    num_cpus: usize,
}

impl ThreadPool{
    pub fn new() -> Self{
        let mut num = num_cpus::get() - 2;

        if num < 2{
            num = 2;
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

            threads.push(
                ThreadHandle{
                    handle: thread,
                    sender: send_s,
                    reciever: recv_s,
                    current_jobs: 0,
                }
                );

        }
        ThreadPool{
            threads: threads,
            num_cpus: num,
            job_id: 0,
        }
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

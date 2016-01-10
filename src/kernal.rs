use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering; use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::thread::spawn;

use std::time::Duration;

use super::Event;
use super::CoreEvent;

use super::profile::ProfileSample;

pub enum SystemOption{
    Syncronized,
    JoinHandleed,
    Optional,
}

pub enum SystemType{
    Render,
    Console,
    Logic,
}

impl SystemType{
    pub fn get_index(&self) -> usize{
        match *self{
            SystemType::Render => 0,
            SystemType::Console => 1,
            SystemType::Logic => 2,
        }
    }
}

pub trait System{
    //fn get_system_option() -> SystemOption;
    //fn get_system_type() -> SystemType;
    fn run(&mut self);
}


pub struct EventHandle{
    recv: Receiver<Event>,
    send: Sender<Event>,
    notify: Arc<Condvar>,
}

impl EventHandle{
    pub fn get_event_blocking(&self) -> Event{
        match self.recv.recv(){
            Ok(x) => x,
            Err(_) => Event::Core(CoreEvent::Quit),
        }
    }

    pub fn get_event(&self) -> Option<Event>{
        match self.recv.try_recv(){
            Ok(x) => Some(x),
            Err(x) => match x {
                TryRecvError::Disconnected => Some(Event::Core(CoreEvent::Quit)),
                TryRecvError::Empty => None,
            }
        }
    }

    /*
     * i am gone need to take a look at
     * this function at a later date.
     */
    pub fn push(&self,event: Event) -> bool{
        match self.send.send(event){
            Ok(_) => {
                self.notify.notify_one();
                true
            },
            Err(_) => false,
        }
    }
}


impl<'a> IntoIterator for &'a mut EventHandle{
    type Item = Event;
    type IntoIter = NonBlockingIter<'a>;

    fn into_iter(self) -> Self::IntoIter{
        NonBlockingIter{
            reciever: &self.recv,
        }
    }
}

pub struct KernalBuilder{
    systems: Vec<Box<System>>,
    senders: Vec<Sender<Event>>,
    recievers: Vec<Receiver<Event>>,
    notify: Arc<Condvar>,
}

impl KernalBuilder{
    pub fn new() -> Self{
        KernalBuilder{
            systems: Vec::new(),
            senders: Vec::new(),
            recievers: Vec::new(),
            notify: Arc::new(Condvar::new()),
        }
    }

    pub fn add_system(&mut self,sys: Box<System>){
        self.systems.push(sys);
    }

    pub fn get_event_handle(&mut self) -> EventHandle{
        let(handle_send,recv) = channel::<Event>();
        let(send,handle_recv) = channel::<Event>();
        let handle = EventHandle{
            recv: handle_recv,
            send: handle_send,
            notify: self.notify.clone(),
        };
        self.senders.push(send);
        self.recievers.push(recv);
        handle
    }

    pub fn build(self) -> Kernal{
        let running = Arc::new(AtomicBool::new(true));
        let mut thread_data = KernalThread{
            recievers: self.recievers,
            senders: self.senders,
            running: running.clone(),
            notify: self.notify,
            stupid_unnecessary_mutex: Mutex::new(()),
        };
        Kernal{
            systems: self.systems,
            thread: thread::spawn(move || thread_data.run()),
            running: running,
        }
    }
}

/*
 * The struct managing the thread
 * handeling management of the events. 
 */
pub struct KernalThread{
    recievers: Vec<Receiver<Event>>,
    senders: Vec<Sender<Event>>,
    running: Arc<AtomicBool>,
    //Maybe it is better to use park unpark.
    //The current implementation needs a unnecessary mutex
    notify: Arc<Condvar>,
    stupid_unnecessary_mutex: Mutex<()>,
    
}


pub struct NonBlockingIter<'a>{
    reciever: &'a Receiver<Event>,
}

impl<'a> Iterator for NonBlockingIter<'a>{
    type Item = Event;

    fn next(&mut self) -> Option<Event>{
        self.reciever.try_recv().ok()
    }
}

impl KernalThread{
    fn run(&mut self){
        trace!("Starting messages thread.");
        let mut i = 0;
        'main: loop{
            {
                ProfileSample::new("Message thread loop");
                if self.recievers.len() > 0 {
                    for e in self.recievers.iter()
                        .flat_map(move |it| NonBlockingIter{reciever: &it}){
                            match e {
                                Event::Core(CoreEvent::Quit) => {
                                    self.running.store(false,Ordering::Relaxed);
                                    break 'main;
                                },
                                _ => {},
                            }
                            trace!("Event: {:?}",e);
                            for sender in &self.senders{
                                sender.send(e.clone()).unwrap();
                            }
                        }
                }
                self.notify.wait(self.stupid_unnecessary_mutex.lock().unwrap()).unwrap();
            }
            i +=1;
            if i % 100 == 0 {
                ProfileSample::print();
                ProfileSample::clear();
            }
        }
        trace!("Quiting messages thread.");
    }
}

pub struct Kernal{
    running: Arc<AtomicBool>,
    systems: Vec<Box<System>>,
    thread: JoinHandle<()>,
}

impl Kernal{
    pub fn run(&mut self){
        trace!("Starting kernal.");
        while self.running.load(Ordering::Relaxed) {
            ProfileSample::new("Kernal loop");
            //collect
            for i in 0..self.systems.len(){
                self.systems[i].run();
            }
            //thread::sleep(Duration::from_millis(10));
        }
        trace!("Quiting kernal.");
    }
}


use std::iter;
use std::sync::Condvar;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::thread::JoinHandle;
use std::thread::spawn;


use std::mem::swap;


use super::Event;
use super::CoreEvent;

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
            Ok(_) => true,
            Err(_) => false,
        }
    }
}


impl<'a> IntoIterator for &'a mut EventHandle{
    type Item = Event;
    type IntoIter = <&'a Receiver<Event> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter{
        (&self.recv).into_iter()
    }
}

pub struct KernalBuilder{
    condvar: Condvar,
    systems: Vec<Box<System>>,
    senders: Vec<Sender<Event>>,
    recievers: Vec<Receiver<Event>>,
}

impl KernalBuilder{
    pub fn new() -> Self{
        KernalBuilder{
            condvar: Condvar::new(),
            systems: Vec::new(),
            senders: Vec::new(),
            recievers: Vec::new(),
        }
    }

    pub fn add_system(mut self,sys: Box<System>) -> Self{
        self.systems.push(sys);
        self
    }

    pub fn get_event_handle(&mut self) -> EventHandle{
        let(handle_send,recv) = channel::<Event>();
        let(send,handle_recv) = channel::<Event>();
        let handle = EventHandle{
            recv: handle_recv,
            send: handle_send,
        };
        self.senders.push(send);
        self.recievers.push(recv);
        handle
    }

    pub fn build(&mut self) -> Kernal{
        let mut thread = KernalThread{
                recievers: self.recievers,
                senders: self.senders,
        };

        Kernal{
            running: true,
            systems: self.systems,
            thread: thread::spawn(move || thread.run()),
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
}

impl KernalThread{
    fn run(&mut self){
        loop{
            if self.recievers.len() > 0 {
                for e in self.recievers.iter().flat_map(move |it| it.iter()){
                    for sender in &self.senders{
                        sender.send(e.clone());
                    }
                }
            }
        }
    }
}

pub struct Kernal{
    running: bool,
    systems: Vec<Box<System>>,
    thread: JoinHandle<()>,
}

impl Kernal{

    pub fn run(&mut self){
        while self.running {
            //collect
            for i in 0..self.systems.len(){
                self.systems[i].run();
            }
        }
    }
}


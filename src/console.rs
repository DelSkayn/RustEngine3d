#![allow(unused_variables)]
use super::log;
use log::{LogRecord, LogMetadata,SetLoggerError,LogLevelFilter};
use log::LogLevel::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::sync::atomic::AtomicBool; use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use super::time;
use std::thread;
use std::io;
use std::collections::HashMap;

use std::mem;

use super::event::{
    EventCreator,
    BaseEvent,
};


pub use log::LogLevel;

pub type LogLevelContainer<T> = Arc<Mutex<Cell<T>>>;

//
// Struct handeling the Console output
// 
// Today i learned a great lesson READ TO DOCS BEFORE YOU IMPLEMENT SOMETHING
//
struct ConsoleLogger;


impl ConsoleLogger{
    fn init() -> Result<(), SetLoggerError>{
        log::set_logger(move |max_log_level| {
            max_log_level.set(LogLevelFilter::Info);
            Box::new(ConsoleLogger)
        })
    }
}

impl log::Log for ConsoleLogger{
    fn enabled(&self, metadata: &LogMetadata) -> bool{
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord){
        if self.enabled(record.metadata()){
            println!("[{}][{}]{}"
                     ,time::now().strftime("T").unwrap()
                     ,record.level()
                     ,record.args());
        }
    }
}

//
//Struct handeling input
//Might be easier with mio but works for now
//Handels all input via console does so on a seperate thread
//so that the game can run while waiting for input on stdin
struct ConsoleInput{
    commands: Arc<Mutex<Vec<String>>>,
    reading: Arc<AtomicBool>, 
}

impl ConsoleInput{
    fn new() -> Self{
        ConsoleInput{
            commands: Arc::new(Mutex::new(Vec::new())),
            reading: Arc::new(AtomicBool::new(false)),
        }
    }

    //
    //Creates the reading thread and starts its work
    //
    fn run(&mut self){
        self.reading.store(true,Ordering::Relaxed);
        let reading = self.reading.clone();
        let commands = self.commands.clone();
        thread::spawn(move ||{
            let io_in = io::stdin();
            while reading.load(Ordering::Relaxed) {
                let mut input = String::new();
                io_in.read_line(&mut input).unwrap();
                commands.lock().unwrap().push(input);
            }
        });
    }
    
    //
    //returns the accumelated messages.
    fn get_message(&mut self) -> Vec<String>{
        let mut lock = self.commands.lock().unwrap();
        let res = lock.clone();
        lock.clear();
        res
    }
}

type ConsoleCommand<T> = Fn(&[&str]) -> Option<T>;

//
//Struct handeling both the input and output and the
//execution of commands from the console
pub struct Console<T = BaseEvent>{
    input: RefCell<ConsoleInput>,
    commands: HashMap<&'static str,Box<ConsoleCommand<T>>>,
    events: RefCell<Vec<T>>,
}


impl<T> Console<T>{
    pub fn new() -> Self{
        let mut input = ConsoleInput::new();
        input.run();
        ConsoleLogger::init().unwrap();
        let mut commands = HashMap::<&'static str,Box<ConsoleCommand<T>>>::new();
        commands.insert("echo",Box::new(
            |args:&[&str]|{
                print!("[echo]");
                for e in args {
                    print!("{}",e);
                }
                println!("");
                None
            }));
        commands.insert("null",Box::new(|args|(None)));
        Console{
            input: RefCell::new(input),
            commands: commands,
            events: RefCell::new(Vec::new()),
        }
    }

    //This seams impossible when using the log lib
    //need to look into
    pub fn set_log_lvl(&mut self,lvl: LogLevel){
        unimplemented!();
    }

    //gets input and executes commands
    //There is one hardcoded command
    //namely 'commands' wich returns all existing commands
    pub fn update(&self){
        let mut events = self.events.borrow_mut();
        for e in self.input.borrow_mut().get_message(){
            let mut split = e.split_whitespace();
            let name = match split.next(){
                None => continue,
                Some(x) => x,
            };
            if name == "commands" {
                for (command,_) in &self.commands{
                    println!("[commands] {}", command);
                }
                continue;
            }
            match self.commands.get(name) {
                None => {
                    println!("Command {} not regonized",name);
                    continue;
                },
                Some(x) => {
                    let args: Vec<_> = split.collect();
                    if let Some(x) = x(&args){
                        events.push(x);
                    }
                }
            };
        }
    }


    //
    //Adds a command to be executed when the entered in the console
    //
    //It might be cool if these could be loaded from a file
    //That would mean that i need some sort of interpeted language or so.
    //Still Cool idea.
    //
    pub fn add_command<F>(&mut self,name: &'static str,func: F)
        where F: Fn(&[&str]) -> Option<T>, F: 'static{
            self.commands.insert(name,Box::new(func));
    }
}

impl<T> EventCreator<T> for Console<T>{
    fn get_events(&self) -> Vec<T>{
        self.update();
        mem::replace(&mut self.events.borrow_mut(),Vec::new())
    }
}


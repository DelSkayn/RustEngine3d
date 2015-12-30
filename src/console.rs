#![allow(unused_variables)]
use super::log;
use log::{LogRecord, LogMetadata,SetLoggerError,LogLevelFilter};
use log::LogLevel::*;

use super::time;

use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::io;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

use std::str::FromStr;
use std::ops::Drop;

use super::Event;
use super::kernal::System;
use super::kernal::EventHandle;


pub use log::LogLevel;

pub type LogLevelContainer<T> = Arc<Mutex<Cell<T>>>;

//
// Struct handeling the Console output
// 
// Today i learned a great lesson READ TO DOCS BEFORE YOU IMPLEMENT SOMETHING
//
struct ConsoleLogger{
    log_channel: Mutex<Sender<String>>,
}



impl ConsoleLogger{
    fn init() -> Result<Receiver<String>, SetLoggerError>{
        let (send,recv) = channel::<String>();
        log::set_logger(move |max_log_level| {
            max_log_level.set(LogLevelFilter::Trace);
            Box::new(ConsoleLogger{
                log_channel: Mutex::new(send),
            })
        }).map(|_| recv)
    }
}

impl log::Log for ConsoleLogger{
    fn enabled(&self, metadata: &LogMetadata) -> bool{
        metadata.level() <= LogLevel::Trace
    }

    fn log(&self, record: &LogRecord){
        if self.enabled(record.metadata()){
            //trace log is not the usefull if it isnt immedeitly printed
            println!("[{}]{}",record.level(),record.args());
            let res = format!("[{}][{}]{}"
                              ,time::now().strftime("%T").unwrap()
                              ,record.level()
                              ,record.args());
            match self.log_channel.lock().unwrap().send(res){
                Ok(_) => {},
                Err(x) => {
                    warn!("Console was deleted! Disabling logging to file");
                }
            }
        }
    }
}

//
//Struct handeling input
//Might be easier with mio but works for now
//Handels all input via console does so on a seperate thread
//so that the game can run while waiting for input on stdin
struct ConsoleInput{
    input_channel: Sender<String>,
}

impl ConsoleInput{
    fn new() -> Receiver<String>{
        let (send,recv) = channel::<String>();
        trace!("Creating Console Input thread!");
        Self::run(send);
        recv
    }

    //
    //Creates the reading thread and starts its work
    //
    fn run(send: Sender<String>){
        thread::Builder::new().name(String::from_str("ConsoleInputThread").unwrap()).spawn(move ||{
            let io_in = io::stdin();
            'main: loop {
                let mut input = String::new();
                io_in.read_line(&mut input).unwrap();
                match send.send(input){
                    Ok(_) => {},
                    Err(x) => {
                        warn!("Console was deleted! Quiting input thread");
                        break 'main;
                    }
                };
            }
        }).unwrap();
    }

}

type ConsoleCommand = Fn(&[&str]) -> Option<Event>;
//
//Struct handeling both the input and output and the
//execution of commands from the console
pub struct Console{
    commands: HashMap<&'static str,Box<ConsoleCommand>>,
    log_channel: Receiver<String>,
    input_channel: Receiver<String>,
    enable_logging: Cell<bool>,
    event: EventHandle,
    log_file: RefCell<File>,
}


impl Console{
    pub fn new(event:EventHandle) -> Self{
        let recv = ConsoleLogger::init().unwrap();
        trace!("Creating Console");
        let recv_in = ConsoleInput::new();
        let mut commands = HashMap::<&'static str,Box<ConsoleCommand>>::new();
        let file = File::create("log.txt").unwrap();


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
            event: event,
            commands: commands,
            log_channel: recv,
            enable_logging: Cell::new(true),
            log_file: RefCell::new(file),
            input_channel: recv_in,
        }
    }

    //This seams impossible when using the log lib
    //need to look into
    pub fn set_log_lvl(&mut self,lvl: LogLevel){
        unimplemented!();
    }

    fn handel_logging(&self){
        loop{
            match self.log_channel.try_recv() {
                Ok(x) => {
                    write!(self.log_file.borrow_mut(),"{}\n",x).unwrap();
                }
                Err(x) => match x{
                    TryRecvError::Empty => break,
                    TryRecvError::Disconnected => println!("Error, logging channel disconnected!"),
                }
            }
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
        where F: Fn(&[&str]) -> Option<Event>, F: 'static{
            self.commands.insert(name,Box::new(func));
        }
}

impl System for Console{
    //gets input and executes commands
    //There is one hardcoded command
    //namely 'commands' wich returns all existing commands
    fn run(&mut self){
        if self.enable_logging.get(){
            self.handel_logging();
        }

        while let Ok(e) = self.input_channel.try_recv(){
            let mut split = e.split_whitespace();
            let name = match split.next(){
                None => continue,
                Some(x) => x,
            };
            if name == "commands" {
                for (command,_) in &self.commands{
                    println!("[commands] {}", command);
                }
                println!("[commands] commands");
                println!("[commands] log");
                continue;
            }
            if name == "log"{
                self.enable_logging.set(!self.enable_logging.get());
            continue;
        }
        match self.commands.get(name) {
            None => {
                println!("Command \"{}\" not regonized",name);
                continue;
            },
            Some(x) => {
                let args: Vec<_> = split.collect();
                if let Some(x) = x(&args){
                    self.event.push(x);
                }
            }
        };
        }
    }
}


impl Drop for Console{
    fn drop(&mut self){
        //log pending messages
        self.run();
    }
}

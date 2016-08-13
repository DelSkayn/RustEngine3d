//! Console.
//!
//! This module specifies types for the developer console.
//! The console is an integral part of Tungsten.
//!
//! When the engine is complete you will be able to inspect verious valeus at runtime,
//! controle systems and get performance profile data through the console.
//!
//! it currently can only be used to quit the engine, execute system commands and do asset stuff.
//!
//! TODO:
//!     
//!     * Fix weird input in child commands using "!".
//!

mod sys_terminal;
pub use self::sys_terminal::SystemTerminal;

mod commands;
use self::commands::*;

use std::collections::HashMap;

use std::process::{Stdio,Command as StdCommand};

use std::str;

use state::State;


pub trait Terminal: 'static {
    fn read(&mut self) -> Vec<String>;
    fn write(&mut self, s: String);
}

type Command<T> = Box<Fn(&[&str], &mut T) + Send>;

pub struct Console<T: Terminal> {
    terminal: T,
    commands: HashMap<String, Command<T>>,
}

impl<T: Terminal> Console<T> {
    pub fn new(terminal: T) -> Self {
        info!("Console ready");
        let mut c = Console {
            terminal: terminal,
            commands: HashMap::new(),
        };
        c.add_command("quit".to_string(), |_, t| {
            t.write("quiting!".to_string());
            State::quit();
        });
        c.add_command("asset".to_string(),asset_command);
        c.add_command("!".to_string(),|args,t|{
            if args.len() < 1{
                t.write("missing arguments!: ! \"command\"".to_string());
                return
            }
            // TODO defer later
            let mut comm = StdCommand::new(args[0]);
            for i in 1..args.len(){
                comm.arg(args[i]);
            }
            println!("{:?}",comm);

            match comm.spawn(){
                Ok(mut x) => 
                {
                    match x.wait(){
                        Ok(_) => {},
                        Err(e) => t.write(format!("Process recieved error: {:?}",e)),
                    }
                },
                Err(e) => {
                    t.write(format!("Process could not execute: {:?}",e));
                },

            };
        });
        c

    }

    pub fn update(&mut self) {
        let commands = self.terminal.read();
        for s in commands {
            let mut comms = s.split_whitespace();
            let command = match comms.next() {
                Some(x) => x,
                None => continue,
            };
            // FIXME find a non allocating solution
            let args: Vec<_> = comms.collect();
            match self.commands.get(command) {
                Some(x) => x(&args, &mut self.terminal),
                None => self.terminal.write("Command not found".to_string()),
            }
        }
    }

    fn add_command<F>(&mut self, name: String, func: F)
        where F: Fn(&[&str], &mut T) + Send + 'static
        {
            self.commands.insert(name, Box::new(func));
        }
}

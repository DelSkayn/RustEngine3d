
use std::sync::mpsc::{self,Sender,Receiver,TryRecvError};
use std::thread;

use std::io::{self,BufRead};

use super::Terminal;

pub struct SystemTerminal{
    recv: Receiver<String>,
}

impl SystemTerminal{
    pub fn new() -> Self{
        info!("Using system console");
        let (send,recv) = mpsc::channel();
        thread::spawn(||thread_loop(send));
        SystemTerminal{
            recv: recv,
        }
    }
}

impl Terminal for SystemTerminal{
    fn read(&mut self) -> Vec<String>{
        let mut res = Vec::new();
        loop{
            match self.recv.try_recv(){
                Ok(x) => {
                    res.push(x);
                }
                Err(e) => {
                    match e{
                        TryRecvError::Empty => break,
                        TryRecvError::Disconnected =>
                            panic!("System terminal input thread disconnected"),
                    }
                }
            }
        }
        res
    }

    fn write(&mut self,s: String){
        println!("{}",&s);
    }
}

//unfortunatly we cant join the io thread
//It is blocked on reading input so joining it 
//will not work.

fn thread_loop(send: Sender<String>){
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    loop{
        let mut string = String::new();
        lock.read_line(&mut string).unwrap();
        match send.send(string) {
            Ok(()) => {},
            Err(_) => break,
        }
    }
}


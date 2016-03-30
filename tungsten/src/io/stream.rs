use std::thread;
use std::thread::JoinHandle;

use std::sync::mpsc::{
    Receiver,
    Sender,
    channel,
}

enum StreamCommand{
    Load(FileId,PathBuf),
    Quit
}

enum StreamMessage{
    Done(FileId,Vec<u8>),
}

pub struct StreamManager{
    thread: JoinHandle<()>,
    sender: Sender<StreamCommand>,
    reciever: Receiver<StreamMessage>,
}

impl StreamManager{
    
    fn new() -> Self{
        let (com_send,com_recv) = channel();
        let (mess_send,mess_recv) = channel();
        StreamManager{
            thread: thread::spawn(||{
                for e in com_recv{
                    match e{
                        Quit => {break},
                        _ => unimplemented!(),
                    }
                }
            })
        }
    }
}

impl Drop for StreamManager{
    fn drop(&mut self){
        self.sender.send(StreamCommand::Quit);
        self.thread.join();
    }
}

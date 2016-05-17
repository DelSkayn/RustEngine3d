use std::path::PathBuf;
use std::thread::JoinHandle;

use std::sync::mpsc::{Sender, Receiver, self}

use super::File as ThreadFile;

enum IoMessage{
    Read(ThreadFile,PathBuf),
    Quit,
}

struct IoHandle{
    handle: JoinHandle<()>,
    send: Sender<IoMessage>,
}

impl IoHandle{
    fn send(im: IoMessage){
        self.send.send(im).expect("Io thread disconnected unexpected");
    }
}

impl Drop for IoHandle{
    fn drop(&mut self){
        self.send.send(IoMessage::Quit).expect("Io thread disconnected unexpected");
        self.handle.join();
    }
}

struct IoThread;

impl IoThread{
    fn new() -> IoHandle{
        let (send,recv) = mpsc::channel();
        let handle = thread::spawn(|| thread_loop(recv));
        IoHandle{
            handle: handle,
            send: send,
        }
    }
}

fn thread_loop(recv: Receiver<IoMessage>){
    loop{
        match recv.recv().expect("Io thread got disconnected unexpected");{
            IoMessage::Read(file,PathBuf) => {
            }
            IoMessage::Quit => {
                debug!("Io thread quiting!");
                break;
            }
        }
    }
}

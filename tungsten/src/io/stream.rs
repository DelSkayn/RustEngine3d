/// module containing the implentation of the stream.

extern crate crossbeam;
use self::crossbeam::sync::MsQueue;

extern crate task;
use task::DynamicPromise;

use super::super::util::FnBox;

use std::thread::JoinHandle;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::collections::HashMap;

use std::fs::File;

use std::io::SeekFrom;
use std::io::Result;
use std::io::Write;
use std::io::Read;
use std::io::Seek;

use std::thread;
use std::time::Duration;


lazy_static!{ static ref STREAM: Stream = Stream::new(); }

/// A unique of a file.
#[derive(Eq,PartialEq,Clone,Copy)]
pub struct FileId(usize);

/// Enum containing all operations.
pub enum Command{
    Read(usize,FileId,Sender<Result<Vec<u8>>>),
    ReadFully(FileId,Sender<Result<Vec<u8>>>),
    ReadCallback(usize,FileId,Box<FnBox<Result<Vec<u8>>,Output = ()> + Send>),
    ReadFullyCallback(FileId,Box<FnBox<Result<Vec<u8>>,Output = ()> + Send>),
    Write(Vec<u8>,FileId,Sender<Result<()>>),
    Seek(SeekFrom,FileId,Sender<Result<()>>),
    Open(File,Sender<FileId>),
    Close(FileId),
    Stop,
}

pub struct Stream{
    join: Option<JoinHandle<()>>,
    que: Arc<MsQueue<Command>>,
}

impl Drop for Stream{
    fn drop(&mut self){
        self.que.push(Command::Stop);
        self.join.take().unwrap().join().expect("Stream thread ended prematurely");
    }
}

impl Stream{
    fn new() -> Self{
        let que = Arc::new(MsQueue::new());
        let c_que = que.clone();
        let join = thread::spawn(|| run(c_que));
        Stream{
            que: que,
            join: Some(join),
        }
    }

    pub fn que(com: Command){
        STREAM.que.push(com);
        Self::wake();
    }

    pub fn wake(){
        STREAM.join.as_ref().unwrap().thread().unpark();
    }
}

/// Runs a thread.
fn run(que: Arc<MsQueue<Command>>){
    let mut open_files = HashMap::new();
    // TODO impl for wrapping integer.
    let mut next = 0usize;
    let mut pending = Vec::new();
    loop{
        let res;
        loop{
            pending.retain(|prom: &DynamicPromise<()>| !prom.done());
            if let Some(que) = que.try_pop(){
                res = que;
                break;
            }else{
                thread::park_timeout(Duration::from_millis(10));
            }
        }
        trace!("Recieved io event");
        match res { 
            Command::Open(file,sender) => {
                let id = next;
                next +=1;
                open_files.insert(id,file);
                sender.send(FileId(id)).unwrap();
            },
            Command::Read(size,file_id,sender) => {
                let FileId(id) = file_id;
                let mut buf = Vec::with_capacity(size);
                for _ in 0..size{
                    buf.push(0);
                }
                let res = open_files.get_mut(&id)
                    .expect("File not opened before reading.")
                    .read(&mut buf);
                match res{
                    Ok(_) => {
                        sender.send(Ok(buf)).unwrap();
                    },
                    Err(x) => {
                        sender.send(Err(x)).unwrap();
                    }
                }
            },
            Command::ReadFully(file_id,sender) => {
                let FileId(id) = file_id;
                let mut buf = Vec::new();
                let res = open_files.get_mut(&id)
                    .expect("File not opened before reading.")
                    .read_to_end(&mut buf);
                match res{
                    Ok(_) => {
                        sender.send(Ok(buf)).unwrap();
                    },
                    Err(x) => {
                        sender.send(Err(x)).unwrap();
                    }
                }
            },
            Command::ReadCallback(size,file_id,callback) => {
                let FileId(id) = file_id;
                let mut buf = Vec::with_capacity(size);
                for _ in 0..size{
                    buf.push(0);
                }
                let res = open_files.get_mut(&id)
                    .expect("File not opened before reading.")
                    .read(&mut buf);
                match res{
                    Ok(_) => {
                        let promise = DynamicPromise::new(|| {
                            callback.call_box(Ok(buf));
                            Stream::wake();
                        });
                        promise.run();
                        pending.push(promise);
                    },
                    Err(x) => {
                        let promise = DynamicPromise::new(|| {
                            callback.call_box(Err(x));
                            Stream::wake();
                        });
                        promise.run();
                        pending.push(promise);
                    }
                }
            },
            Command::ReadFullyCallback(file_id,callback) => {
                let FileId(id) = file_id;
                let mut buf = Vec::new();
                let res = open_files.get_mut(&id)
                    .expect("File not opened before reading.")
                    .read_to_end(&mut buf);
                match res{
                    Ok(_) => {
                        let promise = DynamicPromise::new(|| {
                            callback.call_box(Ok(buf));
                            Stream::wake();
                        });
                        promise.run();
                        pending.push(promise);
                    },
                    Err(x) => {
                        let promise = DynamicPromise::new(|| {
                            callback.call_box(Err(x));
                            Stream::wake();
                        });
                        promise.run();
                        pending.push(promise);
                    }
                }
            },
            Command::Write(buf,file_id,sender) => {
                let FileId(id) = file_id;
                let res = open_files.get_mut(&id)
                    .expect("File not opened before writing.")
                    .write(&buf);
                match res{
                    Ok(_) => {
                        sender.send(Ok(())).unwrap();
                    },
                    Err(x) => {
                        sender.send(Err(x)).unwrap();
                    }
                }
            },
            Command::Seek(seek,file_id,sender) => {
                let FileId(id) = file_id;
                let res = open_files.get_mut(&id)
                    .expect("File not opened before writing.")
                    .seek(seek);
                match res{
                    Ok(_) => {
                        sender.send(Ok(())).unwrap();
                    },
                    Err(x) => {
                        sender.send(Err(x)).unwrap();
                    }
                }
            },
            Command::Close(file_id) => {
                let FileId(id) = file_id;
                if let None = open_files.remove(&id){
                    warn!("Tried to close file which was not opened.");
                }
            },
            Command::Stop => {
                return;
            }
        }
    }
}

extern crate crossbeam;

use self::crossbeam::sync::MsQueue;

use std::thread::JoinHandle;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::collections::HashMap;

use std::path::PathBuf;

use std::fs::File;

use std::io::SeekFrom;
use std::io::Result;
use std::io::Write;
use std::io::Read;
use std::io::Seek;

use std::thread;

lazy_static!{ static ref STREAM: Stream = Stream::new(); }

#[derive(Eq,PartialEq,Clone,Copy)]
pub struct FileId(usize);

pub enum Command{
    Read(usize,FileId,Sender<Result<Vec<u8>>>),
    ReadFully(FileId,Sender<Result<Vec<u8>>>),
    Write(Vec<u8>,FileId,Sender<Result<()>>),
    Seek(SeekFrom,FileId,Sender<Result<()>>),
    Open(PathBuf,Sender<Result<FileId>>),
    Create(PathBuf,Sender<Result<FileId>>),
    Close(FileId),
    Stop,
}

pub struct Stream{
    join: JoinHandle<()>,
    que: Arc<MsQueue<Command>>,
}

impl Stream{
    fn new() -> Self{
        let que = Arc::new(MsQueue::new());
        let c_que = que.clone();
        let join = thread::spawn(|| run(c_que));
        Stream{
            que: que,
            join: join,
        }
    }

    pub fn que(com: Command){
        STREAM.que.push(com);
    }
}

fn run(que: Arc<MsQueue<Command>>){
    let mut open_files = HashMap::new();
    // TODO impl for wrapping integer.
    let mut next = 0usize;
    loop{
        match que.pop(){
            Command::Open(path,sender) => {
                let file = File::open(path);
                match file{
                    Ok(x) => {
                        let id = next;
                        next +=1;
                        open_files.insert(id,x);
                        sender.send(Ok(FileId(id))).unwrap();
                    },
                    Err(e) => {
                        sender.send(Err(e)).unwrap();
                    }
                }
            },
            Command::Create(path,sender) => {
                let file = File::create(path);
                match file{
                    Ok(x) => {
                        let id = next;
                        next +=1;
                        open_files.insert(id,x);
                        sender.send(Ok(FileId(id))).unwrap();
                    },
                    Err(e) => {
                        sender.send(Err(e)).unwrap();
                    }
                }
            },
            Command::Read(size,file_id,sender) => {
                let FileId(id) = file_id;
                let mut buf = Vec::with_capacity(size);
                for _ in 0..size{
                    buf.push(0);
                }
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
            Command::ReadFully(file_id,sender) => {
                let FileId(id) = file_id;
                let mut buf = Vec::new();
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

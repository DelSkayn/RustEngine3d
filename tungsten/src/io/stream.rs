extern crate crossbeam;

use self::crossbeam::sync::MsQueue;
use std::sync::Arc;

use std::io::Read;
use std::io::Write;

use std::path::Path;
use std::path::PathBuf;
use std::thread::{JoinHandle,self};
use super::File as ThreadFile;

use std::fs::File;
use std::io::ErrorKind;

use super::Result;
use super::Error;

pub enum IoMessage{
    Read(ThreadFile,PathBuf),
    Create(ThreadFile,PathBuf),
    Write(ThreadFile,PathBuf),
    Quit,
}

pub struct IoHandle{
    handle: Option<JoinHandle<()>>,
    send: Arc<MsQueue<IoMessage>>,
}

impl IoHandle{
    pub fn send(&self,im: IoMessage){
        self.send.push(im);
    }
}

impl Drop for IoHandle{
    fn drop(&mut self){
        self.send(IoMessage::Quit);
        self.handle.take().unwrap().join().expect("Io thread quit before join!");
    }
}

pub struct IoThread;

impl IoThread{
    pub fn new() -> IoHandle{
        let que = Arc::new(MsQueue::new());
        let clone_que = que.clone();
        let handle = thread::spawn(move || thread_loop(clone_que));
        IoHandle{
            handle: Some(handle),
            send: que,
        }
    }
}

fn thread_loop(que: Arc<MsQueue<IoMessage>>){
    loop{
        match que.pop(){
            IoMessage::Read(file,path) => {
                file.0.done(read(&path));
            },
            IoMessage::Write(file,path) => {
                unsafe{
                    let buf = (*file.0.file.get()).take().unwrap().unwrap();
                    match write(buf,&path){
                        Err(e) => {(*file.0.file.get()) = Some(Err(e))},
                        _ => {}
                    }
                }
            },
            IoMessage::Create(file,path) => {
                unsafe{
                    let buf = (*file.0.file.get()).take().unwrap().unwrap();
                    match create(buf,&path){
                        Err(e) => {(*file.0.file.get()) = Some(Err(e))},
                        _ => {}
                    }
                }
            },
            IoMessage::Quit => {
                debug!("Io thread quiting!");
                break;
            },
        }
    }
}

fn read(path: &Path) -> Result<Vec<u8>>{
    let mut file = try!(File::open(path).map_err(|e|{
        match e.kind(){
            ErrorKind::NotFound => Error::NotFound,
            _ => Error::Other,
        }
    }));
    let meta = try!(file.metadata().map_err(|_|{
        Error::Other
    }));
    if meta.is_dir(){
        return Err(Error::NotAFile);
    }
    let mut buf = Vec::with_capacity(meta.len() as usize);
    try!(file.read_to_end(&mut buf).map_err(|_|{
        Error::Other
    }));
    Ok(buf)
}
fn write(buff: Vec<u8>,path: &Path) -> Result<()>{
    let mut file = try!(File::open(path).map_err(|e|{
        match e.kind(){
            ErrorKind::NotFound => Error::NotFound,
            _ => Error::Other,
        }
    }));
    let meta = try!(file.metadata().map_err(|_|{
        Error::Other
    }));
    if meta.is_dir(){
        return Err(Error::NotAFile);
    }
    try!(file.write_all(&buff).map_err(|_|{
        Error::Other
    }));
    Ok(())
}

fn create(buff: Vec<u8>,path: &Path) -> Result<()>{
    let mut file = try!(File::create(path).map_err(|_|{
        Error::Other
    }));
    let meta = try!(file.metadata().map_err(|_|{
        Error::Other
    }));
    if meta.is_dir(){
        return Err(Error::NotAFile);
    }
    try!(file.write_all(&buff).map_err(|_|{
        Error::Other
    }));
    Ok(())
}

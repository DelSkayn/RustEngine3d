extern crate crossbeam;

use self::crossbeam::sync::MsQueue;
use std::sync::Arc;

use std::io::Read;
use std::io::Write;

use std::path::Path;
use std::path::PathBuf;

use std::env;
use std::thread::{self, JoinHandle};
use std::fs::File;
use std::io::ErrorKind;

use super::FileWrite;
use super::FileRead;
use super::Result;
use super::Error;

pub enum IoMessage {
    Read(FileRead, PathBuf),
    Create(FileWrite, Vec<u8>, PathBuf),
    Write(FileWrite, Vec<u8>, PathBuf),
    Quit,
}

pub struct IoHandle {
    handle: Option<JoinHandle<()>>,
    send: Arc<MsQueue<IoMessage>>,
}

impl IoHandle {
    pub fn send(&self, im: IoMessage) {
        self.send.push(im);
    }
}

impl Drop for IoHandle {
    fn drop(&mut self) {
        self.send(IoMessage::Quit);
        self.handle.take().unwrap().join().expect("Io thread quit before join!");
    }
}

pub struct IoThread;

impl IoThread {
    pub fn new() -> IoHandle {
        let que = Arc::new(MsQueue::new());
        let clone_que = que.clone();
        let handle = thread::spawn(move || thread_loop(clone_que));
        IoHandle {
            handle: Some(handle),
            send: que,
        }
    }
}

fn thread_loop(que: Arc<MsQueue<IoMessage>>) {
    info!("Starting io thread.");
    info!("Execution dir: \"{}\".",
          env::current_dir()
              .unwrap()
              .to_str()
              .unwrap());
    loop {
        match que.pop() {
            IoMessage::Read(file, path) => {
                file.0.file.complete(read(&path));
                file.0.sleep.awake();
            }
            IoMessage::Write(file, data, path) => {
                file.0.file.complete(write(data, &path));
                file.0.sleep.awake();
            }
            IoMessage::Create(file, data, path) => {
                file.0.file.complete(create(data, &path));
                file.0.sleep.awake();
            }
            IoMessage::Quit => {
                debug!("Io thread quiting!");
                break;
            }
        }
    }
}

fn read(path: &Path) -> Result<Vec<u8>> {
    let mut file = try!(File::open(path).map_err(|e| {
        match e.kind() {
            ErrorKind::NotFound => Error::NotFound,
            _ => Error::Other,
        }
    }));
    let meta = try!(file.metadata().map_err(|_| Error::Other));
    if meta.is_dir() {
        return Err(Error::NotAFile);
    }
    let mut buf = Vec::with_capacity(meta.len() as usize);
    try!(file.read_to_end(&mut buf).map_err(|_| Error::Other));
    Ok(buf)
}

fn write(buff: Vec<u8>, path: &Path) -> Result<()> {
    let mut file = try!(File::open(path).map_err(|e| {
        match e.kind() {
            ErrorKind::NotFound => Error::NotFound,
            _ => Error::Other,
        }
    }));
    let meta = try!(file.metadata().map_err(|_| Error::Other));
    if meta.is_dir() {
        return Err(Error::NotAFile);
    }
    try!(file.write_all(&buff).map_err(|_| Error::Other));
    Ok(())
}

fn create(buff: Vec<u8>, path: &Path) -> Result<()> {
    let mut file = try!(File::create(path).map_err(|e| {
        match e.kind() {
            ErrorKind::NotFound => Error::NotFound,
            _ => Error::Other,
        }
    }));
    let meta = try!(file.metadata().map_err(|_| {
        println!("2");
        Error::Other
    }));
    if meta.is_dir() {
        return Err(Error::NotAFile);
    }
    try!(file.write_all(&buff).map_err(|_| {
        println!("3");
        Error::Other
    }));
    Ok(())
}

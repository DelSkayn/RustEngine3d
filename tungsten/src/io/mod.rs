use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use std::cell::UnsafeCell;

use std::path::Path;
use std::ffi::OsStr;

use std::thread::{Thread, self};

use std::convert::Into;

mod stream;
use self::stream::IoThread;
use self::stream::IoHandle;
use self::stream::IoMessage;

lazy_static!{
    static ref IO: Io = Io::new();
}

use std::result::Result as StdResult;

pub type Result<T> = StdResult<T,Error>;

#[derive(Debug)]
pub enum Error{
    NotFound,
    AlreadExists,
    NotAFile,
    Other,
}


struct FileInner{
    file: UnsafeCell<Option<Result<Vec<u8>>>>,
    complete: AtomicBool,
    block: AtomicBool,
    thread: UnsafeCell<Option<Thread>>,
}

unsafe impl Send for FileInner{}
unsafe impl Sync for FileInner{}

impl FileInner{
    fn new() -> Self{
        FileInner{
            file: UnsafeCell::new(None),
            complete: AtomicBool::new(false),
            block: AtomicBool::new(false),
            thread: UnsafeCell::new(None),
        }
    }

    fn with_buff(vec: Vec<u8>) -> Self{
        FileInner{
            file: UnsafeCell::new(Some(Ok(vec))),
            complete: AtomicBool::new(false),
            block: AtomicBool::new(false),
            thread: UnsafeCell::new(None),
        }
    }

    fn sleep(&self){
        unsafe{
            println!("called");
            (*self.thread.get()) = Some(thread::current());
            self.block.store(true,Ordering::Release);
            if !self.complete.load(Ordering::Acquire){
                thread::park();
            }
            self.block.store(false,Ordering::Release);
        }
    }

    fn wake(&self){
        if self.block.load(Ordering::Acquire){
            let t = unsafe{(*self.thread.get()).take().unwrap()};
            while self.block.load(Ordering::Acquire){
                t.unpark();
            }
        }
    }

    fn done(&self,f: Result<Vec<u8>>){
        unsafe{
            *self.file.get() = Some(f);
        }
        self.complete.store(true,Ordering::Release);
        self.wake();
    }
}

#[derive(Clone)]
pub struct File(Arc<FileInner>);

impl File{
    pub fn is_done(&self) -> bool{
        self.0.complete.load(Ordering::Acquire)
    }

    pub fn done_data(self) -> Result<Vec<u8>>{
        if !self.is_done(){
            self.0.sleep();
        }
        unsafe{
            (*self.0.file.get()).take().unwrap()
        }
    }

    pub fn done(self) -> Result<()>{
        unsafe{
            if !self.is_done(){
                self.0.sleep();
            }
            match (*self.0.file.get()).take(){
                Some(x) => {
                    x.map(|_| ())
                },
                None => {Ok(())},
            }
        }
    }
}

impl Drop for File{
    fn drop(&mut self){
        if !self.is_done(){
            self.0.sleep();
        }
        unsafe{
            (*self.0.file.get()).take()
                .map(|e|{
                    e.ok()
                        .expect("Dropped file handle returned an error");
                });
        }
    }
}

pub struct Io{
    join: IoHandle,
}

impl Io{
    fn new() -> Self{
        Io{
            join: IoThread::new(),
        }
    }
}

impl Io{
    pub fn read<S: AsRef<OsStr>>(path: S) -> File{
        let path = Path::new(path.as_ref()).to_path_buf();
        let file = File(Arc::new(FileInner::new()));
        IO.join.send(IoMessage::Read(file.clone(),path));
        file
    }

    pub fn write<S: AsRef<OsStr>,B: Into<Vec<u8>>>(path: S,vec: B) -> File{
        let path = Path::new(path.as_ref()).to_path_buf();
        let file = File(Arc::new(FileInner::with_buff(vec.into())));
        IO.join.send(IoMessage::Write(file.clone(),path));
        file
    }

    pub fn create<S: AsRef<OsStr>,B: Into<Vec<u8>>>(path: S,vec: B) -> File{
        let path = Path::new(path.as_ref()).to_path_buf();
        let file = File(Arc::new(FileInner::with_buff(vec.into())));
        IO.join.send(IoMessage::Create(file.clone(),path));
        file
    }
}


use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use std::io::Error;

use std::mem;
use std::cell::UnsafeCell;

mod thread;
use self::thread::IoThread;

lazy_static!{
    static IO: Io = Io::new();
}

struct FileInner{
    file: UnsafeCell<Result<Vec<u8>,Error>>,
    complete: AtomicBool,
    block: AtomicBool,
    thread: UnsafeCell<Thread>,
}

impl FileInner{
    fn new() -> Self{
        unsafe{
            FileInner{
                file: UnsafeCell::new(mem::uninitialized()),
                complete: AtomicBool::new(false),
                block: AtomicBool::new(false),
                thread: UnsafeCell::new(mem::unimplemented()),
            }
        }
    }

    fn sleep(&self){
        unsafe{
            (*self.thread.get()) = thread::current();
            self.block.store(true,Ordering::Release);
            if !self.complete.load(Ordering::Acquire){
                thread::park();
            }
            self.block.store(false,Ordering::Release);
        }
    }

    fn wake(&self){
        while self.block.load(Ordering::Acquire){
            self.thread.unpark();
        }
    }

    fn done(&self,f: FileForm){
        (*self.file.get()) = f;
        self.complete.store(true,Ordering::Release);
        self.wake();
    }
}


pub struct File(Arc<FileInner>);

impl File{
    pub fn is_done(&self) -> bool{
        self.0.complete.load(Ordering::Acquire);
    }

    pub fn into_file(self) -> Result<Vec<u8>,IoError>{
        if !self.is_done(){
            self.0.sleep();
        }
        self.0.file
    }
}

pub struct Io{
    join: JoinHandle<()>,
}

impl Io{
    fn new() -> Self{
        Io{
            join: 
        }
    }
}


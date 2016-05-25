//! IO
//!
//! Tungsten use streaming IO for its loading of resources.
//! This module specifies the types nessecary for stream io.
//!
//! TODO:
//!  * Find a way so that you dont have to load the entire file.
//!  * Do some api cleaning. the current implementation is very rough.
//!

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::cell::UnsafeCell;

use std::path::Path;
use std::ffi::OsStr;

use std::thread::{self, Thread};

use std::convert::Into;

mod stream;
use self::stream::IoThread;
use self::stream::IoHandle;
use self::stream::IoMessage;

lazy_static!{
    static ref IO: Io = Io::new();
}

use std::result::Result as StdResult;

#[derive(Debug)]
pub enum Error {
    NotFound,
    AlreadExists,
    NotAFile,
    Other,
}

pub type Result<T> = StdResult<T, Error>;


struct ReadData {
    file: UnsafeCell<Option<Result<Vec<u8>>>>,
    complete: AtomicBool,
}

struct SleepData {
    thread: UnsafeCell<Option<Thread>>,
    block: AtomicBool,
}

struct WriteData {
    complete: AtomicBool,
    file: UnsafeCell<Option<Result<()>>>,
}

unsafe impl Send for ReadData {}
unsafe impl Sync for ReadData {}

unsafe impl Send for SleepData {}
unsafe impl Sync for SleepData {}

unsafe impl Send for WriteData {}
unsafe impl Sync for WriteData {}

impl ReadData {
    fn new() -> Self {
        ReadData {
            file: UnsafeCell::new(None),
            complete: AtomicBool::new(false),
        }
    }

    fn done(&self) -> bool {
        self.complete.load(Ordering::Acquire)
    }

    fn complete(&self, res: Result<Vec<u8>>) {
        unsafe{ (*self.file.get()) = Some(res) };
        self.complete.store(true, Ordering::Release);
    }

    fn as_inner(&self) -> Result<Vec<u8>> {
        if !self.done() {
            panic!("Tried to unwrap file before it was loaded");
        }
        unsafe { (*self.file.get()).take().unwrap() }
    }
}

impl WriteData {
    fn new() -> Self {
        WriteData {
            file: UnsafeCell::new(None),
            complete: AtomicBool::new(false),
        }
    }

    fn done(&self) -> bool {
        self.complete.load(Ordering::Acquire)
    }

    fn complete(&self, res: Result<()>) {
        unsafe { (*self.file.get()) = Some(res) };
        self.complete.store(true, Ordering::Release);
    }

    fn as_inner(&self) -> Result<()> {
        if !self.done() {
            panic!("Tried to unwrap file before it was loaded");
        }
        unsafe { (*self.file.get()).take().unwrap() }
    }
}

impl SleepData {
    fn new() -> Self {
        SleepData {
            thread: UnsafeCell::new(None),
            block: AtomicBool::new(false),
        }
    }

    fn sleep(&self) {
        self.block.store(true, Ordering::Release);
        unsafe { (*self.thread.get()) = Some(thread::current()) };
        thread::park();
        self.block.store(false, Ordering::Release);
    }

    fn awake(&self) {
        while self.block.load(Ordering::Acquire) {
            unsafe {
                if let Some(thread) = (*self.thread.get()).take() {
                    thread.unpark();
                }
            }
        }
    }
}

pub struct FileWriteInner {
    sleep: SleepData,
    file: WriteData,
}

impl FileWriteInner {
    fn new() -> Self {
        FileWriteInner {
            sleep: SleepData::new(),
            file: WriteData::new(),
        }
    }
}

impl FileReadInner {
    fn new() -> Self {
        FileReadInner {
            sleep: SleepData::new(),
            file: ReadData::new(),
        }
    }
}

pub struct FileReadInner {
    sleep: SleepData,
    file: ReadData,
}

pub struct FileWrite(Arc<FileWriteInner>);

pub struct FileRead(Arc<FileReadInner>);

impl FileWrite {
    pub fn is_done(&self) -> bool {
        self.0.file.done()
    }

    pub fn into_inner(self) -> Result<()> {
        if !self.0.file.done() {
            self.0.sleep.sleep();
        }
        self.0.file.as_inner()
    }
}

impl FileRead {
    pub fn is_done(&self) -> bool {
        self.0.file.done()
    }

    pub fn into_inner(self) -> Result<Vec<u8>> {
        if !self.0.file.done() {
            self.0.sleep.sleep();
        }
        self.0.file.as_inner()
    }
}

pub struct Io {
    join: IoHandle,
}

impl Io {
    fn new() -> Self {
        Io { join: IoThread::new() }
    }
}

impl Io {
    pub fn read<S: AsRef<OsStr>>(path: S) -> FileRead {
        let path = Path::new(path.as_ref()).to_path_buf();
        let file = Arc::new(FileReadInner::new());
        IO.join.send(IoMessage::Read(FileRead(file.clone()), path));
        FileRead(file)
    }

    pub fn write<S: AsRef<OsStr>, B: Into<Vec<u8>>>(path: S, vec: B) -> FileWrite {
        let path = Path::new(path.as_ref()).to_path_buf();
        let file = Arc::new(FileWriteInner::new());
        IO.join.send(IoMessage::Write(FileWrite(file.clone()), vec.into(), path));
        FileWrite(file)
    }

    pub fn create<S: AsRef<OsStr>, B: Into<Vec<u8>>>(path: S, vec: B) -> FileWrite {
        let path = Path::new(path.as_ref()).to_path_buf();
        let file = Arc::new(FileWriteInner::new());
        IO.join.send(IoMessage::Create(FileWrite(file.clone()), vec.into(), path));
        FileWrite(file)
    }
}

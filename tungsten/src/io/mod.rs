//! Streaming io module.
//!
//! Tungsten use streaming io for its loading of resources.
//! This module specifies the types nessecary for stream io.
//!
//! All operations are pushed to a thread wich will execute them.
//! Allowing to do io while being able to keep running.
//!
//! I honestly have no idea if this is even usefull.
//!
//! ### TODO:<br/>
//!
//!     * Find a way so that you dont have to load the entire file.
//!     * Do some api cleaning. the current implementation is very rough.
//!     * Dont open files on a thread.
//!

mod stream;

use self::stream::{FileId,Stream,Command};

//use task::{self,DynTaskImpl,Latch,ArcLatch};

use std::sync::mpsc::{Receiver,TryRecvError,self};
use std::path::Path;
pub use std::io::Result;

/// Used for keeping track of the current state of the file.
/// Because currently the file is also opened on the thread 
/// we need to wait till the thread returned a open file.
enum FileState{
    Ready(FileId),
    Wait(Receiver<Result<FileId>>),
}

impl FileState{
    // TODO change wen lifeliness is implemented.
    fn get_id(&mut self) -> FileId{
        let id = match self{
            &mut FileState::Ready(ref x) => return x.clone(),
            &mut FileState::Wait(ref recv) => {
                recv.recv().unwrap().unwrap()
            },
        };
        *self = FileState::Ready(id);
        id
    }

    fn wait(&mut self) -> Result<()>{
        let id = match self{
            &mut FileState::Wait(ref recv) => {
                try!(recv.recv().unwrap())
            },
            _ => return Ok(()),
        };
        *self = FileState::Ready(id);
        Ok(())
    }
}

/// Struct containing the result of an io operation with a callback.
/// If the struct is dropped before an result was retrieved, it will block the thread until a
/// result is available and if the io had an error it will panic.
pub struct CallbackResult<R>(Option<Receiver<Result<R>>>);

impl<R> CallbackResult<R>{
    /// Try to get the result if it is available.
    pub fn try(&mut self) -> Option<Result<R>>{
        match self.0.as_mut().unwrap().try_recv(){
            Ok(x) => Some(x),
            Err(e) => match e {
                TryRecvError::Empty => None,
                _ => panic!("Io thread disconnected!"),
            }
        }
    }
    
    /// Wait until the result is available and return it.
    pub fn wait(&mut self) -> Result<R>{
        self.0.take().unwrap().recv().unwrap()
    }
}

impl<R> Drop for CallbackResult<R>{
    fn drop(&mut self){
        if let Some(ref x) = self.0{
            x.recv().unwrap()
                .expect("CallbackResult returned an error");
        }
    }
}


/// Struct containing the result of an write operation
/// If the struct is dropped before an result was retrieved, it will block the thread until a
/// result is available and if the io had an error it will panic.
pub struct WriteResult(Option<Receiver<Result<()>>>);

impl WriteResult{
    /// Try to get the result if it is available.
    pub fn try(&mut self) -> Option<Result<()>>{
        match self.0.as_mut().unwrap().try_recv(){
            Ok(x) => Some(x),
            Err(e) => match e {
                TryRecvError::Empty => None,
                _ => panic!("Io thread disconnected!"),
            }
        }
    }

    /// Wait until the result is available and return it.
    pub fn wait(mut self) -> Result<()>{
        self.0.take().unwrap().recv().unwrap()
    }
}

impl Drop for WriteResult{
    fn drop(&mut self){
        if let Some(ref x) = self.0{
            x.recv().unwrap()
                .expect("File write returned an error.");
        }
    }
}

/// Struct containing the result of an read operation
/// If the struct is dropped before an result was retrieved, it will block the thread until a
/// result is available and if the io had an error it will panic.
pub struct ReadResult(Option<Receiver<Result<Vec<u8>>>>);

impl ReadResult{
    /// Try to get the result if it is available.
    pub fn try(&mut self) -> Option<Result<Vec<u8>>>{
        match self.0.as_mut().unwrap().try_recv(){
            Ok(x) => Some(x),
            Err(e) => match e {
                TryRecvError::Empty => None,
                _ => panic!("Io thread disconnected!"),
            }
        }
    }

    /// Wait until the result is available and return it.
    pub fn wait(mut self) -> Result<Vec<u8>>{
        self.0.take().unwrap().recv().unwrap()
    }
}

impl Drop for ReadResult{
    fn drop(&mut self){
        if let Some(ref x) = self.0{
            x.recv().unwrap()
                .expect("File write returned an error.");
        }
    }
}

// Correct because you need a mutable ref to do anything.
unsafe impl<R> Sync for CallbackResult<R>{}
unsafe impl Sync for ReadResult{}
unsafe impl Sync for WriteResult{}

/// A struct representing a file which can be read from.
pub struct File(FileState);

impl File{
    /// Open a file at path 
    /// us ready to recieve posible errors.
    pub fn open<P: AsRef<Path>>(path: P) -> Self{
        let (send,recv) = mpsc::channel();
        Stream::que(Command::Open(path.as_ref().to_path_buf(),send));
        File(FileState::Wait(recv))
    }

    /// Open a file at path, if it does not exist it will create it.
    /// us ready to recieve posible errors.
    pub fn create<P: AsRef<Path>>(path: P) -> Self{
        let (send,recv) = mpsc::channel();
        Stream::que(Command::Create(path.as_ref().to_path_buf(),send));
        File(FileState::Wait(recv))
    }

    /// Wait until the file is ready.
    /// Returns posible errors.
    /// It is advised to call ready before calling other functions.
    pub fn ready(&mut self) -> Result<()>{
        self.0.wait()
    }

    /// Write data to the file.
    /// When called and the file is not ready it will wait until it is.
    /// If when opening a file an error occured the function will panic.
    pub fn write(&mut self,data: &[u8]) -> WriteResult{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        let mut buf = Vec::with_capacity(data.len());
        buf.extend_from_slice(data);
        Stream::que(Command::Write(buf,id,send));
        WriteResult(Some(recv))
    }

    /// Read data up to amount bytes.
    /// When called and the file is not ready it will wait until it is.
    /// If when opening a file an error occured the function will panic.
    pub fn read(&mut self,amount: usize) -> ReadResult{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        Stream::que(Command::Read(amount,id,send));
        ReadResult(Some(recv))
    }

    /// Read all data from a file to the end.
    /// When called and the file is not ready it will wait until it is.
    /// If when opening a file an error occured the function will panic.
    pub fn read_to_end(&mut self) -> ReadResult{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        Stream::que(Command::ReadFully(id,send));
        ReadResult(Some(recv))
    }

    /// Read data from a file and pass the data into a callback.
    /// When called and the file is not ready it will wait until it is.
    /// If when opening a file an error occured the function will panic.
    pub fn read_callback<R: Send + 'static,F: FnOnce(Vec<u8>) -> R + Send + 'static>(&mut self,amount: usize,func: F) -> CallbackResult<R>{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        Stream::que(Command::ReadCallback(amount,id,Box::new(move |data: Result<Vec<u8>>|{
            match data{
                Ok(x) => send.send(Ok(func(x))).unwrap(),
                Err(e) => send.send(Err(e)).unwrap(),
            }
        })));
        CallbackResult(Some(recv))
    }

    /// Read a entire file and pass the data into a callback.
    /// When called and the file is not ready it will wait until it is.
    /// If when opening a file an error occured the function will panic.
    pub fn read_to_end_callback<R: Send + 'static,F: FnOnce(Vec<u8>) -> R + Send + 'static>(&mut self,func: F) -> CallbackResult<R>{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        Stream::que(Command::ReadFullyCallback(id,Box::new(move |data: Result<Vec<u8>>|{
            match data{
                Ok(x) => send.send(Ok(func(x))).unwrap(),
                Err(e) => send.send(Err(e)).unwrap(),
            }
        })));
        CallbackResult(Some(recv))
    }
}



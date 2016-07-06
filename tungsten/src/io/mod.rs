//! IO
//!
//! Tungsten use streaming IO for its loading of resources.
//! This module specifies the types nessecary for stream io.
//!
//! TODO:
//!  * Find a way so that you dont have to load the entire file.
//!  * Do some api cleaning. the current implementation is very rough.
//!

mod stream;

use std::sync::mpsc::{Receiver,self};

use self::stream::{FileId,Stream,Command};

use std::path::Path;

use std::io::Result;

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

pub struct WriteResult(Option<Receiver<Result<()>>>);

impl WriteResult{
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

pub struct ReadResult(Option<Receiver<Result<Vec<u8>>>>);

impl ReadResult{
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

pub struct File(FileState);

impl File{
    pub fn open<P: AsRef<Path>>(path: P) -> Self{
        let (send,recv) = mpsc::channel();
        Stream::que(Command::Open(path.as_ref().to_path_buf(),send));
        File(FileState::Wait(recv))
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Self{
        let (send,recv) = mpsc::channel();
        Stream::que(Command::Create(path.as_ref().to_path_buf(),send));
        File(FileState::Wait(recv))
    }

    pub fn ready(&mut self) -> Result<()>{
        self.0.wait()
    }

    pub fn write(&mut self,data: &[u8]) -> WriteResult{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        let mut buf = Vec::with_capacity(data.len());
        buf.extend_from_slice(data);
        Stream::que(Command::Write(buf,id,send));
        WriteResult(Some(recv))
    }

    pub fn read(&mut self,amount: usize) -> ReadResult{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        Stream::que(Command::Read(amount,id,send));
        ReadResult(Some(recv))
    }

    pub fn read_to_end(&mut self) -> ReadResult{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        Stream::que(Command::ReadFully(id,send));
        ReadResult(Some(recv))
    }
}



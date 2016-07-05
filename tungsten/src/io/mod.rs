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

use std::sync::mpsc::{Sender,Receiver,self};

use self::stream::{FileId,Stream,Command};

use std::path::Path;

use std::io::Result;

enum FileState{
    Ready(FileId),
    Wait(Receiver<Result<FileId>>),
}

impl FileState{
    fn get_id(&mut self) -> FileId{
        match self{
            &mut FileState::Ready(x) => x,
            &mut FileState::Wait(recv) => {
                let id = recv.recv().unwrap().unwrap();
                *self = FileState::Ready(id);
                id
            },
        }
    }

    fn wait(&mut self) -> Result<()>{
        match self{
            FileState::Wait(recv) => {
                let id = try!(recv.recv().unwrap());
                *self = FileState::Ready(id);
            },
            _ => Ok(()),
        }
    }
}

struct WriteResult(Receiver<Result<()>>);

impl WriteResult{
    fn wait(&mut self) -> Result<()>{
        self.0.recv().unwrap()
    }
}

impl Drop for WriteResult{
    fn drop(&mut self){
        self.0.recv().unwrap().expect("File write returned an error.");
    }
}

struct ReadResult(Receiver<Result<Vec<u8>>>);

impl ReadResult{
    fn wait(&mut self) -> Result<Vec<u8>>{
        self.0.recv().unwrap()
    }
}

impl Drop for ReadResult{
    fn drop(&mut self){
        self.0.recv().unwrap().expect("File write returned an error.");
    }
}

struct File(FileState);

impl File{
    fn open<P: AsRef<Path>>(path: P) -> Self{
        let (send,recv) = mpsc::channel();
        Stream::que(Command::Open(path.to_path_buf(),send));
        File(FileState::Wait(recv))
    }

    fn create<P: AsRef<Path>>(path: P) -> Self{
        let (send,recv) = mpsc::channel();
        Stream::que(Command::Create(path.to_path_buf(),send));
        File(FileState::Wait(recv))
    }

    fn ready(&mut self) -> Result<()>{
        self.0.wait()
    }

    fn write(&mut self,data: &[u8]) -> WriteResult{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        let buf = Vec::with_capacity(data.len());
        buf.extend_from_slice(data);
        Stream::que(Command::Write(data,id,send));
        WriteResult(recv)
    }

    fn read(&mut self,amount: usize) -> ReadResult{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        Stream::que(Command::Read(amount,id,send));
        ReadResult(recv)
    }

    fn read_to_end(&mut self) -> ReadResult{
        let (send,recv) = mpsc::channel();
        let id = self.0.get_id();
        Stream::que(Command::ReadFully(id,send));
        ReadResult(recv)
    }
}



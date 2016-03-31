use std::collections::HashMap;
use std::collections::VecDeque;

use std::cell::RefCell;

use std::path::PathBuf;

use std::env;

use super::System;
use super::Root;
use super::JobBuilder;

mod stream;
use self::stream::StreamManager;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct FileId(u64);

pub enum IOError{
    PathNotInRes,
    NotImplemented,
}

pub struct IOData{
    local_dir: PathBuf,
    internal: RefCell<InternalIOData>,
}

impl IOData{
    pub fn new() -> Self{
        IOData{
            local_dir: env::current_dir().unwrap(),
            internal: RefCell::new(
                InternalIOData{
                    files: HashMap::new(),
                    load_queue: VecDeque::new(),
                }),
        }
    }

    pub fn load(&self,path: PathBuf) -> Result<FileId,IOError>{
        Err( IOError::NotImplemented)
    }
}

struct InternalIOData{
    files: HashMap<FileId,FileData>,
    load_queue: VecDeque<(FileId,PathBuf)>,
}

pub struct FileData{
    path: PathBuf,
    file: Option<FileForm>,
}

pub enum FileForm{
    Str(String),
    Raw(Vec<u8>),
}

struct IOSystem{
    stream: StreamManager,
}

impl IOSystem{
    fn new() -> Self{
        IOSystem{
            stream: StreamManager::new(),
        }
    }
}

impl System for IOSystem{
    fn run(&mut self,root: &Root) -> Option<JobBuilder>{

        None
    }
}

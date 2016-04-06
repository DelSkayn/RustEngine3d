use std::collections::VecDeque;
use std::collections::HashMap;

use std::cell::RefCell;

use std::path::PathBuf;
use std::path::Path;

use std::env;

use super::System;
use super::Root;
use super::JobBuilder;

use std::hash::Hasher;

use super::util::HashAlgo;
use super::util::NoHashBuilder;

mod stream;
use self::stream::StreamManager;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct FileId(u64);

impl FileId{
    fn from_path(path: &Path) -> Self{
        // TODO actually hash this when a hasher is implemented;
        // The current implementation only uses the first 
        // four bytes of the path.
        let lower_path = &path.to_str().unwrap().to_lowercase();
        let mut hasher = HashAlgo::new();
        hasher.write(lower_path.as_bytes());
        FileId(hasher.finish())
    }

    fn inner(&self) -> u64{
        self.0
    }
}

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
            local_dir: env::current_dir().unwrap(),//should not be here... maybe
            internal: RefCell::new(
                InternalIOData{
                    files: HashMap::with_hasher(NoHashBuilder::new()),
                    load_queue: VecDeque::new(),
                }),
        }
    }

    pub fn load(&self,path: PathBuf) -> Result<FileId,IOError>{
        let id = FileId::from_path(&path);
        let mut data = self.internal.borrow_mut();

        data.load_queue.push_back((id.clone(),path.clone()));

        data.files.insert(id.inner(),FileData{
            path: path,
            file: None,
        });
        Err(IOError::NotImplemented)
    }

}

struct InternalIOData{
    files: HashMap<u64,FileData,NoHashBuilder>,
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
    fn run(&mut self,_root: &Root) -> Option<JobBuilder>{
        None
    }
}

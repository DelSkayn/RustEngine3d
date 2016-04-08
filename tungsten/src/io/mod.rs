use std::collections::HashMap;

use std::path::PathBuf;
use std::path::Path;

use std::io::Error;
use std::io::ErrorKind;


use std::hash::Hasher;

use super::util::HashAlgo;
use super::util::NoHashBuilder;

mod stream;
use self::stream::StreamManager;
use self::stream::StreamMessage;
use self::stream::StreamCommand;

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

#[derive(Debug)]
pub enum IOError{
    PathNotInRes,
    NotImplemented,
    FileDoesNotExist,
    PermissionDeneid,
    NotAFile,
    Other,
}

impl IOError{

    fn from_error(error: Error) -> Self{
        match error.kind(){
            ErrorKind::NotFound => IOError::FileDoesNotExist,
            ErrorKind::PermissionDenied => IOError::PermissionDeneid,
            _ => IOError::Other,
        }

    }
}

pub struct IOData{
    files: HashMap<u64,FileData,NoHashBuilder>,
    stream: StreamManager,
}

impl IOData{
    pub fn new() -> Self{
        IOData{
            //local_dir: env::current_dir().unwrap(),//should not be here... maybe
            files: HashMap::with_hasher(NoHashBuilder::new()),
            stream: StreamManager::new(),
        }
    }

    pub fn load(&mut self,path: PathBuf) -> Result<FileId,IOError>{
        let id = FileId::from_path(&path);
        self.files.insert(id.inner(),FileData{
            path: path.clone(),
            file: None,
        });
        self.stream.send(StreamCommand::Load(id.clone(),path));
        Ok(id)
    }

    pub fn get(&mut self,id: FileId) -> &FileData{
        unimplemented!()
    }

    fn cycle(&mut self){
        while let Some(x) = self.stream.get(){
            match x {
                StreamMessage::Done(id,data) =>{
                    self.files.get_mut(&id.inner()).unwrap()
                        .file = Some(FileForm::Raw(data));
                },
                StreamMessage::DoneStr(id,data) =>{
                    self.files.get_mut(&id.inner()).unwrap()
                        .file = Some(FileForm::Str(data));
                },
                StreamMessage::Error(id,error) => {
                    self.files.get_mut(&id.inner()).unwrap()
                        .file = Some(FileForm::Error(error));
                },
            }
        }
    }
}

pub struct FileData{
    path: PathBuf,
    file: Option<FileForm>,
}

pub enum FileForm{
    Str(String),
    Raw(Vec<u8>),
    Error(IOError),
}


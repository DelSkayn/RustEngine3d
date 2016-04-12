use std::collections::HashMap;

use std::thread;

use std::sync::RwLock;
use std::sync::Arc;

use std::convert::AsRef;

use std::path::PathBuf;
use std::path::Path;

use std::io::Error;
use std::io::ErrorKind;

use std::hash::Hasher;

use super::util::HashAlgo;
use super::util::NoHashBuilder; 

mod stream;
use self::stream::Stream;
use self::stream::LoadData;
use self::stream::StreamCommand;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct FileId(u64);

lazy_static!{
    static ref FILES: RwLock<HashMap<u64,Arc<FileForm>,NoHashBuilder>> = RwLock::new(HashMap::with_hasher(NoHashBuilder::new()));
}


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

pub struct Io;

pub fn load<P: AsRef<Path>>(path: P) -> FileId{
    let id = FileId::from_path(path.as_ref());
    Stream::send(StreamCommand::Load(LoadData{
        id: id.clone(),
        path: path.as_ref().to_path_buf(),
        park: None,
    }));
    id
}

pub fn load_str<P: AsRef<Path>>(path: P) -> FileId{
    let id = FileId::from_path(&path.as_ref());
    Stream::send(StreamCommand::LoadStr(LoadData{
        id: id.clone(),
        path: path.as_ref().to_path_buf(),
        park: None,
    }));
    id
}

pub fn load_wait<P: AsRef<Path>>(path: P) -> FileId{
    let id = FileId::from_path(path.as_ref());
    Stream::send(StreamCommand::Load(LoadData{
        id: id.clone(),
        path: path.as_ref().to_path_buf(),
        park: Some(thread::current()),
    }));
    thread::park();
    id
}

pub fn load_wait_str<P: AsRef<Path>>(path: P) -> FileId{
    let id = FileId::from_path(path.as_ref());
    Stream::send(StreamCommand::LoadStr(LoadData{
        id: id.clone(),
        path: path.as_ref().to_path_buf(),
        park: Some(thread::current()),
    }));
    thread::park();
    id
}

pub fn get(id: FileId) -> Option<Arc<FileForm>>{
    FILES.read().expect("Files lock poisened!!").get(&id.inner()).map(|v| v.clone())
}

pub fn remove(id: FileId) -> Option<Arc<FileForm>>{
    FILES.write().expect("Files lock poisened!!").remove(&id.inner())
}


pub enum FileForm{
    Str(String),
    Raw(Vec<u8>),
    Error(IOError),
}

#[cfg(test)]
mod test{
    use super::*;

    use std::path::Path;

    static TEST_FILE_DATA: &'static str = r#"This is a test file for testing.
if this file is removed or changed the io test
in io/mod.rs will fail.
Please dont change this file.
Also dont change the variable containing this
text in io/mod.rs or else the test will fail.
"#;

    #[test]
    fn test_str_load(){
        let mut io = IoManager::new();
        let id = io.load_str(Path::new("res/test_file.txt").to_path_buf());
        match io.get_wait(id).file.as_ref().unwrap(){
            &FileForm::Str(ref data) =>{
                assert!(data == TEST_FILE_DATA);
            },
            &FileForm::Raw(_) =>{
                panic!("Recieved wrong format!");
            },
            &FileForm::Error(ref e) =>{
                panic!("Error during reading: {:?}",e);
            },
        }
    }

    #[test]
    fn test_load(){
        let mut io = IoManager::new();
        let id = io.load(Path::new("res/test_file.txt").to_path_buf());
        match io.get_wait(id).file.as_ref().unwrap(){
            &FileForm::Str(_) =>{
                panic!("Recieved wrong format!");
            },
            &FileForm::Raw(ref data) =>{
                assert!(data.as_slice() == TEST_FILE_DATA.as_bytes());
            },
            &FileForm::Error(ref e) =>{
                panic!("Error during reading: {:?}",e);
            },
        }
    }
}


use std::thread;
use std::thread::Thread;

use std::sync::Arc;

use std::sync::atomic::Ordering;

use std::convert::AsRef; 
use std::path::PathBuf;
use std::path::Path;

use std::io::Error;
use std::io::ErrorKind;

use std::hash::Hasher;

use super::util::AtomicOption;

mod stream;
use self::stream::Stream;
use self::stream::StreamCommand;
use self::stream::LoadData;
use self::stream::WriteData;

/*
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
*/

pub enum FileForm{
    Str(String),
    Raw(Vec<u8>),
    Error(IOError),
}

pub enum WriteForm{
    Str(String),
    Raw(Vec<u8>),
}

impl FileForm{

    pub fn as_str(self) -> Result<String,IOError>{
        match self{
            FileForm::Raw(_) => {
                panic!("Incorrect FileForm match");
            },
            FileForm::Str(s) => {
                Ok(s)
            },
            FileForm::Error(e) => {
                Err(e)
            }
        }
    }

    pub fn as_raw(self) -> Result<Vec<u8>,IOError>{
        match self{
            FileForm::Raw(r) => {
                Ok(r)
            },
            FileForm::Str(_) => {
                panic!("Incorrect FileForm match");
            },
            FileForm::Error(e) => {
                Err(e)
            }
        }
    }
}

#[derive(Clone)]
pub struct FileHook{
    file: Arc<AtomicOption<FileForm>>,
}

impl FileHook{
    fn new() -> Self{
        FileHook{
            file: Arc::new(AtomicOption::new()),
        }
    }

    fn load(&self,file: FileForm){
        self.file.swap(file,Ordering::Release);
    }

    pub fn is_loaded(&self) -> bool{
        self.file.is_some(Ordering::Acquire)
    }

    pub fn get(&self) -> Option<FileForm>{
        self.file.take(Ordering::Acquire)
    }

}

/*
   lazy_static!{
   static ref FILES: RwLock<HashMap<u64,Arc<FileForm>,NoHashBuilder>> = RwLock::new(HashMap::with_hasher(NoHashBuilder::new()));
   }*/

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

pub fn load<P: AsRef<Path>>(path: P) -> FileHook{
    let hook = FileHook::new();
    Stream::send(StreamCommand::Load(LoadData{
        path: path.as_ref().to_path_buf(),
        park: None,
        hook: hook.clone(),
    }));
    hook
}

pub fn load_str<P: AsRef<Path>>(path: P) -> FileHook{
    let hook = FileHook::new();
    Stream::send(StreamCommand::LoadStr(LoadData{
        path: path.as_ref().to_path_buf(),
        park: None,
        hook: hook.clone(),
    }));
    hook
}

pub fn load_wait<P: AsRef<Path>>(path: P) -> FileHook{
    let hook = FileHook::new();
    Stream::send(StreamCommand::Load(LoadData{
        path: path.as_ref().to_path_buf(),
        park: Some(thread::current()),
        hook: hook.clone(),
    }));
    thread::park();
    hook
}

pub fn load_wait_str<P: AsRef<Path>>(path: P) -> FileHook{
    let hook = FileHook::new();
    Stream::send(StreamCommand::LoadStr(LoadData{
        path: path.as_ref().to_path_buf(),
        park: Some(thread::current()),
        hook: hook.clone(),
    }));
    thread::park();
    hook
}

pub fn write<P: AsRef<Path>>(path: P,data: Vec<u8>){
    Stream::send(StreamCommand::Write(WriteData{
        path: path.as_ref().to_path_buf(),
        data: WriteForm::Raw(data),
    }));
}

pub fn write_str<P: AsRef<Path>>(path: P,data: String){
    Stream::send(StreamCommand::Write(WriteData{
        path: path.as_ref().to_path_buf(),
        data: WriteForm::Str(data),
    }));
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

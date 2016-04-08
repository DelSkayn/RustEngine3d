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

pub struct IoManager{
    files: HashMap<u64,FileData,NoHashBuilder>,
    stream: StreamManager,
}

impl IoManager{
    pub fn new() -> Self{
        IoManager{
            //local_dir: env::current_dir().unwrap(),//should not be here... maybe
            files: HashMap::with_hasher(NoHashBuilder::new()),
            stream: StreamManager::new(),
        }
    }

    pub fn load(&mut self,path: PathBuf) -> FileId{
        let id = FileId::from_path(&path);
        self.files.insert(id.inner(),FileData{
            path: path.clone(),
            file: None,
        });
        self.stream.send(StreamCommand::Load(id.clone(),path));
        id
    }

    pub fn load_str(&mut self,path: PathBuf) -> FileId{
        let id = FileId::from_path(&path);
        self.files.insert(id.inner(),FileData{
            path: path.clone(),
            file: None,
        });
        self.stream.send(StreamCommand::LoadStr(id.clone(),path));
        id
    }

    pub fn get(&mut self,id: FileId) -> &mut FileData{
        self.cycle();
        self.files.get_mut(&id.inner()).unwrap()
    }

    pub fn get_wait(&mut self,id: FileId) -> &mut FileData{
        self.cycle_wait(&id);
        self.files.get_mut(&id.inner()).unwrap()
    }

    pub fn remove(&mut self,id: FileId) -> Option<FileData>{
        self.files.remove(&id.inner())
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

    fn cycle_wait(&mut self,wait_id: &FileId){
        loop{
            let res = self.stream.get_wait();
            match res {
                StreamMessage::Done(id,data) =>{
                    self.files.get_mut(&id.inner()).unwrap()
                        .file = Some(FileForm::Raw(data));
                    if id == *wait_id{
                        break;
                    }
                },
                StreamMessage::DoneStr(id,data) =>{
                    self.files.get_mut(&id.inner()).unwrap()
                        .file = Some(FileForm::Str(data));
                    if id == *wait_id{
                        break;
                    }
                },
                StreamMessage::Error(id,error) => {
                    self.files.get_mut(&id.inner()).unwrap()
                        .file = Some(FileForm::Error(error));
                    if id == *wait_id{
                        break;
                    }
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

use std::path::PathBuf;
use std::path::Path;

use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

use std::collections::HashMap;

use std::str::FromStr;


pub type FileResult<T> = Result<T,FileError>;

#[derive(Debug)]
pub enum FileError{
    RootNotAFolder,
    NotAFile,
    NotImported,
    PathDoesNotExist,
    IoError,
}

impl From<Error> for FileError{
    fn from(err: Error)-> FileError{
        match err.kind() {
            ErrorKind::NotFound => FileError::PathDoesNotExist,
            _ => FileError::IoError,
        }
    }
}

pub struct FileManager{
    root: PathBuf,
    imported_files: HashMap<String,ManFile>,
}

struct ManFile{
    name: String,
    path: PathBuf,
}

impl FileManager{
    pub fn new<P: AsRef<Path>>(path: P) -> FileResult<FileManager>{
        let file = try!(File::open(path.as_ref().clone()));
        let meta = try!(file.metadata());
        if meta.is_file(){
            return Err(FileError::RootNotAFolder);
        }
        Ok(FileManager{
            root: path.as_ref().to_path_buf(),
            imported_files: HashMap::new(),
        })
    }

    pub fn import_file<S,P>(&mut self,name: S,path: P)-> FileResult<()>
        where S: AsRef<str>,P: AsRef<Path>{

        let file = try!(File::open(self.root.join(path.as_ref())));
        let meta = try!(file.metadata());
        if meta.is_dir() {
            return Err(FileError::NotAFile);
        }

        let name_string = String::from_str(name.as_ref()).unwrap();
        let new_file = ManFile{
            name:name_string.clone(),
            path: path.as_ref().to_path_buf(),
        };
        self.imported_files.insert(name_string,new_file);
        Ok(())
    }

    pub fn get_file<S: AsRef<str>>(&self,name: S) -> FileResult<File>{
        match self.imported_files
            .get(&String::from_str(name.as_ref()).unwrap()){
                Some(file) => {
                    File::open(self.root.join(&file.path))
                        .map_err(|err| FileError::from(err))
                },
                None => return Err(FileError::NotImported),
            }
    }
}


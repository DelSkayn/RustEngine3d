use std::fs::File;
use std::fs::Path;
use std::io::Error;

#[derive(Debug)]
struct FileFolder{
    pub folder: Vec<FileFolder>,
    pub files: Vec<String>,
    pub path: Path,
}

struct FileSystem{
    pub root: FileFolder,
}

enum FileSystemError{
    RootDirNotAFolder,
    OsIOError,
}

impl Into<Error> for FileSystemError{
    fn into(err: Error) -> FileSystemError{
        FileSystemError::OsIOError
    }
}

impl FileSystem{
    fn new(path: Path) -> Result<Self,FileSystemError>{
        if !try!(path.metadata()).is_dir(){
            return Err(FileSystemError::RootDirNotAFolder);
        }else{
        }
    }

struct FileManager{
}

use std::fs::File;
use std::fs::Path;
use std::io::Error;
use std::fs::read_dir;

#[derive(Debug)]
struct FileFolder{
    pub folder: Vec<FileFolder>,
    pub files: Vec<String>,
    pub path: Path,
}

struct FileSystem{
    pub root: FileFolder,
    pub root_path: Path,
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
        }
        let files = try!(read_dir(path));
        for entry in files{
        }
    }
}


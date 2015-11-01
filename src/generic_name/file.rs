use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::io::Error;
use std::io::ErrorKind;
use std::fs;
use std::ffi::OsStr;
use std::collections::HashMap;

#[derive(Debug)]
enum FileType{
    Mesh(MeshFormat),
    Texture(TextureFormat),
    TextureAnimation,
    Font,
}

#[derive(Debug)]
enum MeshFormat{
    Obj,
}

#[derive(Debug)]
enum TextureFormat{
    Png,
    Jpeg,
    Bmp,
}

#[derive(Debug)]
struct FileData{
    name: String,
    path: String,
    file_type: FileType,
}

struct FileSystem{
    files: HashMap<(),FileData>,
    root: String,
} 
enum FileSystemError{
    NotAFile,
    PathNotFound,
    RootNotADir,
    OsIoError,
    FileFormatNotSupported,
}

impl From<Error> for FileSystemError{
    fn from(err: Error) -> Self{
        let err = err.kind();
        match err{
            ErrorKind::NotFound => FileSystemError::PathNotFound,
            _ => FileSystemError::OsIoError,
        }
    }
}

impl FileSystem{
    fn new(path: String) -> Result<Self,FileSystemError>{
        let meta = try!(fs::metadata(&path));
        if !meta.is_dir() {
            return Err(FileSystemError::RootNotADir);
        }
        Ok(FileSystem{
            root: path,
            files: HashMap::new(),
        })
    }

    fn register_file(&mut self,path: String,file_type: FileType) -> Result<(),FileSystemError>{
        let file_path = Path::new(&self.root).join(&path);
        if !try!(fs::metadata(&path)).is_file(){
            debug!("Did not found a file at path \n    {}",path);
            return Err(FileSystemError::NotAFile);
        }
        let file_name = file_path.file_name();
        Ok(())
    }
}


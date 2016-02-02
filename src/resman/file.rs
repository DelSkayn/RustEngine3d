use std::path::PathBuf;
use std::path::Path;

use std::fs::File; use std::io::Error; use std::io::ErrorKind;

use std::collections::HashMap;

use std::str::FromStr;

#[derive(Debug,PartialEq)]
pub enum FileType{
    Unkown,
    Model(ModelFormat),
    Texture(TextureFormat),
    Zip(Box<FileType>),
}

#[derive(Debug,PartialEq)]
pub enum ModelFormat{
    Wavefront,
    Collada,
}

#[derive(Debug,PartialEq)]
pub enum TextureFormat{
    Jpeg,
    Bitmap,
}

pub type FileResult<T> = Result<T,FileError>;

#[derive(Debug,PartialEq)]
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
    pub name: String,
    pub path: PathBuf,
    pub file_type:FileType,
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
            file_type: Self::get_file_format(file),    
        };
        self.imported_files.insert(name_string,new_file);
        Ok(())
    }

    pub fn import_file_with_type<S,P>(&mut self,name: S,path: P
                                      ,file_type: FileType)-> FileResult<()>
        where S: AsRef<str>,P: AsRef<Path>{

        let file = try!(File::open(self.root.join(path.as_ref())));
        let meta = try!(file.metadata());
        if meta.is_dir() {
            return Err(FileError::NotAFile);
        }

        //TODO: check the if the type is correct
        let name_string = name.as_ref().to_string();
        let new_file = ManFile{
            name:name_string.clone(),
            path: path.as_ref().to_path_buf(),
            file_type: file_type,
        };
        self.imported_files.insert(name_string,new_file);
        Ok(())
    }

    pub fn get_file<'a,S: AsRef<str>>(&'a self,name: S) -> FileResult<&'a ManFile>{
        match self.imported_files
            .get(&name.as_ref().to_string()){
                Some(file) => {
                    Ok(file)
                },
                None => return Err(FileError::NotImported),
            }
    }

    /*
     * TODO: implement function
     */
    #[allow(unused_variables)]
    fn get_file_format(file: File) -> FileType{
        FileType::Unkown
    }
}


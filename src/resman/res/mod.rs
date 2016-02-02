use std::collections::HashMap;
use std::io::Error;

use super::file::*;
use super::super::thread_pool::ThreadPool;

use std::fs::File;
use std::str::FromStr;

use std::rc::Rc;

mod obj;

pub type ResResult<T> = Result<T,ResError>;

#[derive(Debug)]
pub enum ResError{
    NotImported,
    NotFound,
    /*
     * File format could not be found.
     */
    UnkownFormat,
    /*
     * File format should be known but file 
     * does not comply with format rules.
     */
    InvalidFormat(&'static str),

    IOError,
}

impl From<FileError> for ResError{
    fn from(fe: FileError) -> ResError{
        match fe{
            FileError::NotImported => ResError::NotImported,
            _ => {panic!("FileManager returned unexpected error")},
        }
    }
}

impl From<Error> for ResError{
    fn from(_: Error)-> ResError{
        ResError::IOError
    }
}

pub trait ResourceData{
    fn get_size(&self) -> usize;
}

trait ResourceFormat<T> : ResourceData{
    fn get_format(&self) -> T;
}


trait ResourceLoader{
    type Format;
    fn new(file: File) -> Self;
    fn load(&mut self) -> ResResult<Self::Format>;
}

pub struct ResourceHandle{
    res: Rc<Option<Box<ResourceData>>>,
}

impl ResourceHandle{
    fn new(rd: Rc<Option<Box<ResourceData>>>) -> ResourceHandle{
        ResourceHandle{
            res: rd,
        }
    }

    fn is_loaded(&self) -> bool{
        self.res.is_some()
    }

    //problem How do we handel data formats.
    //For instance a model file can also include a 
    //material, this cant be properly shown in a [u8]
    fn get_data<'a>(&'a self) -> Option<&'a[u8]>{
        unimplemented!()
//        self.res.map(|e| e.get_data())
    }
}

pub struct ResourceManager<'a>{
    res: HashMap<String,Rc<Option<Box<ResourceData>>>>,
    thread_pool: Option<&'a ThreadPool>,
}

impl<'a> ResourceManager<'a>{
    pub fn new(pool: Option<&'a ThreadPool>) -> Self{
        ResourceManager{
            res: HashMap::new(),
            thread_pool: pool,
        }
    }

    pub fn load<S>(&mut self,name: S,file_man: &FileManager) -> 
        ResResult<ResourceHandle> where S:AsRef<str>{

        let file = try!(file_man.get_file(name.as_ref().clone()));
        //check if we can load the file
        if file.file_type == FileType::Unkown{
            return Err(ResError::UnkownFormat);
        }
        //Create a Rc for the file
        let file_res = Rc::new(None);
        self.res.insert(String::from_str(name.as_ref()).unwrap(),file_res.clone());
        let fsfile = try!(File::open(file.path.clone()));
        self.load_match(fsfile,&file.file_type);
        Ok(ResourceHandle::new(file_res))
    }

    fn load_match(&mut self,file: File,format: &FileType){
        match *format {
            FileType::Model(ModelFormat::Wavefront) => self.load_with::<obj::ObjLoader>(file),
            _ => {unimplemented!()}
        }
    }

    fn load_with<T: ResourceLoader>(&self,file: File){
        match self.thread_pool {
            Some(_) => {
            },
            None =>{
                let mut loader = T::new(file);
                loader.load().unwrap();
            }
        }
    }

    pub fn get<S: AsRef<str>>(&mut self,name: S) -> Option<ResourceHandle>{
        self.res.get(&String::from_str(name.as_ref()).unwrap())
            .map(|x| ResourceHandle::new(x.clone()))
    }

    pub fn update(&mut self){

    }
}

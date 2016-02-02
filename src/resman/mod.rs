mod file;
mod res;


use self::file::FileManager;
use self::file::FileError as FError;

use self::res::ResourceHandle;
use self::res::ResourceManager;
use self::res::ResError;

use super::thread_pool::ThreadPool;

use std::path::PathBuf;
use std::path::Path;

use std::env;

pub type RMResult<T> = Result<T,RMError>;

#[derive(Debug)]
pub enum RMError{
    FileError(FError),
    ResourceError(ResError),
}

impl From<FError> for RMError{
    fn from(fe: FError) -> RMError{
        RMError::FileError(fe)
    }
}

impl From<ResError> for RMError{
    fn from(re: ResError) -> RMError{
        RMError::ResourceError(re)
    }
}

pub struct ResMan<'a>{
    fman: FileManager,
    rman: ResourceManager<'a>,
}


pub struct ResManBuilder<'a>{
    thread_pool: Option<&'a ThreadPool>,
    root: Option<PathBuf>,
}

impl<'a> ResManBuilder<'a>{
    pub fn new() -> Self{
        ResManBuilder{
            root: None,
            thread_pool: None,
        }
    }

    pub fn with_root<P: AsRef<Path>>(mut self,path :P) -> Self{
        self.root = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn with_thread_pool(mut self,thread: &'a ThreadPool) -> Self{
        self.thread_pool = Some(thread);
        self
    }

    pub fn build(self) -> RMResult<ResMan<'a>>{
        let root = match self.root {
            Some(x) => {
                if x.is_relative(){
                    x.join(env::current_dir().unwrap());
                }
                x
            },
            None => {
                env::current_dir().unwrap()
            }

        };
        let fm = try!(FileManager::new(root));
        Ok(ResMan{
            rman: ResourceManager::new(self.thread_pool),
            fman: fm,
        })
    }
}

impl<'a> ResMan<'a>{
    fn get<S>(&mut self,name: S) -> Option<ResourceHandle>
        where S: AsRef<str>{
            self.rman.get(name)
    }

    fn load<S>(&mut self,name: S) -> RMResult<ResourceHandle>
        where S: AsRef<str>{
            match String::from_str(name.as_ref()) {
                try!(self.rman.load(name,&self.fman))
            }
    }

    fn import<S,P>(&mut self,path: P) -> RMResult<()>
        where S: AsRef<str>,P: AsRef<Path>{
        self.fman.import_file(path.as_ref().to_str().unwrap()
                              ,path.as_ref().clone())
            .map_err(|err| RMError::from(err))
    }

    fn import_name<S,P>(&mut self,name: S,path: P) -> RMResult<()>
        where S: AsRef<str>,P: AsRef<Path>{
        self.fman.import_file(name,path).map_err(|err| RMError::from(err))
    }

    fn update(&mut self){
        unimplemented!();
    }
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test_resman(){
        let mut resman = ResManBuilder::new().with_root("res/").build().unwrap();
        resman.import_name("Teapot","teapot.obj").unwrap();
    }
}

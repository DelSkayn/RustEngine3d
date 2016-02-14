mod file;
use self::file::FileManager;
use self::file::FileError as FError;

use super::thread_pool::ThreadPool;

use std::path::PathBuf;
use std::path::Path;

use std::env;

pub type ResResult<T> = Result<T,ResError>;

#[derive(Debug)]
pub enum ResError{
    FileError(FError),
}

impl From<FError> for ResError{
    fn from(fe: FError) -> ResError{
        ResError::FileError(fe)
    }
}

pub struct ResMan<'a>{
    thread_pool: Option<&'a ThreadPool>,
    fman: FileManager,
}

/*
pub struct ResourceHandle<T: Resource>{
    id: String,
    res: Rc<Option<Resource>>,
}*/

pub struct ResManBuilder<'a>{
    thread_pool: Option<&'a ThreadPool>,
    root: Option<PathBuf>,
}

impl<'a> ResManBuilder<'a>{
    pub fn new() -> Self{
        ResManBuilder{
            thread_pool: None,
            root: None,
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

    pub fn build(self) -> ResResult<ResMan<'a>>{
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
            thread_pool: self.thread_pool,
            fman: fm,
        })
    }
}

impl<'a> ResMan<'a>{

    fn import<S,P>(&mut self,name: S,path: P) -> ResResult<()>
        where S: AsRef<str>,P: AsRef<Path>{
        self.fman.import_file(name,path).map_err(|err| ResError::from(err))
    }

}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test_resman(){
        let mut resman = ResManBuilder::new().with_root("res/").build().unwrap();
        resman.import("Teapot","teapot.obj").unwrap();
    }
}

use std::thread;
use std::thread::Thread; 
use std::thread::JoinHandle; 
use std::cell::UnsafeCell;

use std::fs::File;

use std::path::Path;
use std::path::PathBuf;

use std::io::Read;
use std::io::Write;


use super::FileForm;
use super::FileHook;
use super::WriteForm;
use super::IOError;


use std::sync::mpsc::{
    Receiver,
    Sender,
    channel,
};

thread_local!(static SENDER: UnsafeCell<Option<Sender<StreamCommand>>> 
              = UnsafeCell::new(None));

lazy_static!{
    static ref SENDER_REF: HackishRecieverRef = HackishRecieverRef::new();
}

pub enum StreamCommand{
    Load(LoadData),
    LoadStr(LoadData),
    Write(WriteData),
    Quit,
}

pub struct LoadData{
    pub path: PathBuf,
    pub park: Option<Thread>,
    pub hook: FileHook,
}

pub struct WriteData{
    pub path: PathBuf,
    pub data: WriteForm,
}


struct HackishRecieverRef{
    thread: Option<JoinHandle<()>>,
    sender: Sender<StreamCommand>,
}

impl HackishRecieverRef{
    pub fn new() -> Self{
        let (com_send,com_recv) = channel();
        HackishRecieverRef{
            thread: Some(thread::spawn(||{
                FileStream::run(com_recv);
            })),
            sender: com_send,
        }
    }
}

impl Drop for HackishRecieverRef{
    fn drop(&mut self){
        self.sender.send(StreamCommand::Quit).unwrap();
        self.thread.take().unwrap().join().unwrap();
    }
}

unsafe impl Sync for HackishRecieverRef{}


struct FileStream;

impl FileStream {
    fn run(reciever: Receiver<StreamCommand>){
        for e in reciever{
            match e{
                StreamCommand::Quit => {break},
                StreamCommand::Load(f) => Self::load(f),
                StreamCommand::LoadStr(f) => Self::load_str(f),
                StreamCommand::Write(d) => Self::write(d),
                //_ => unimplemented!(),
            }
        }
    }

    fn load(f: LoadData){
        match Self::get_file(&f.path) {
            Ok(mut file) => {
                let mut buf = Vec::new();
                match file.read_to_end(&mut buf){
                    Ok(_) => {},
                    Err(err) => {
                        warn!("Error while reading file {:?}",err);
                        let err = IOError::from_error(err);
                        f.hook.load(FileForm::Error(err));
                        if let Some(ref t) = f.park{
                            t.unpark();
                        }
                        return;
                    }
                };
                f.hook.load(FileForm::Raw(buf));
                if let Some(ref t) = f.park{
                    t.unpark();
                }
            },
            Err(err) => {
                f.hook.load(FileForm::Error(err));
                if let Some(ref t) = f.park{
                    t.unpark();
                }
            },
        }
    }

    fn load_str(f: LoadData){
        match Self::get_file(&f.path){
            Ok(mut file) => {
                let mut buf = String::new();
                match file.read_to_string(&mut buf){
                    Ok(_) => {},
                    Err(err) => {
                        error!("Error while reading file {:?}",err);
                        let err = IOError::from_error(err);
                        f.hook.load(FileForm::Error(err));
                        if let Some(ref t) = f.park{
                            t.unpark();
                        }
                        return;
                    }
                };
                f.hook.load(FileForm::Str(buf));
                if let Some(ref t) = f.park{
                    t.unpark();
                }
            },
            Err(err) => {
                f.hook.load(FileForm::Error(err));
                if let Some(ref t) = f.park{
                    t.unpark();
                }
            },
        }
    }

    fn write(d: WriteData){
        match d.data{
            WriteForm::Str(s) => {
                match Self::open_file(&d.path){
                    Ok(mut file) => {
                        file.write_all(s.as_bytes()).unwrap();
                    },
                    Err(e) => {
                        warn!("we should handle this {:?}",e);
                        return;
                    },
                }
            },
            WriteForm::Raw(r) => {
                match Self::open_file(&d.path){
                    Ok(mut file) => {
                        file.write_all(&r).unwrap();
                    },
                    Err(e) => {
                        warn!("we should handle this {:?}",e);
                        return;
                    },
                }
            },
        }
    }

    fn get_file(p: &Path) -> Result<File,IOError>{
        let file = match File::open(p.clone()){
            Ok(file) => file,
            Err(e) => {
                let err = IOError::from_error(e);
                return Err(err);
            }
        };
        let metadata = match file.metadata(){
            Ok(meta) => meta,
            Err(e) => {
                let err = IOError::from_error(e);
                return Err(err);
            }
        };
        if metadata.is_dir(){
            return Err(IOError::NotAFile);
        }
        return Ok(file);
    }

    fn open_file(p: &Path) -> Result<File,IOError>{
        let file = match File::create(p.clone()){
            Ok(file) => file,
            Err(e) => {
                let err = IOError::from_error(e);
                return Err(err);
            }
        };
        let metadata = match file.metadata(){
            Ok(meta) => meta,
            Err(e) => {
                let err = IOError::from_error(e);
                return Err(err);
            }
        };
        if metadata.is_dir(){
            return Err(IOError::NotAFile);
        }
        return Ok(file);
    }

}

pub struct Stream;

impl Stream{
    pub fn send(sc: StreamCommand){
        SENDER.with(|value|{
            unsafe{
                if let None = *value.get(){
                    *value.get() = Some(SENDER_REF.sender.clone());
                }
                (*value.get()).as_ref().unwrap().send(sc).expect("IO thread disconnected while sending!");
            }
        });
    }
}


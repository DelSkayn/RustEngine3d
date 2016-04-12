use std::thread;
use std::thread::Thread;
use std::thread::JoinHandle;

use std::cell::UnsafeCell;

use std::fs::File;

use std::path::PathBuf;
use std::io::Read;


use std::sync::Arc;

use super::FileId;
use super::FileForm;
use super::IOError;
use super::FILES;

use std::sync::mpsc::{
    Receiver,
    Sender,
    channel,
    TryRecvError,
};

thread_local!(static SENDER: UnsafeCell<Option<Sender<StreamCommand>>> = UnsafeCell::new(None));

lazy_static!{
    static ref SENDER_REF: HackishRecieverRef = HackishRecieverRef::new();
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

#[derive(Debug)]
pub struct LoadData{
    pub id: FileId,
    pub path: PathBuf,
    pub park: Option<Thread>,
}

impl LoadData{
    fn finish(&self){
    }
}

struct FileStream;

impl FileStream {
    fn run(reciever: Receiver<StreamCommand>){
        for e in reciever{
            match e{
                StreamCommand::Quit => {break},
                StreamCommand::Load(f) => Self::load(f),
                StreamCommand::LoadStr(f) => Self::load_str(f),
                //_ => unimplemented!(),
            }
        }
    }

    fn load(f: LoadData){
        if let Some(mut file) = Self::get_file(&f){
            let mut buf = Vec::new();
            match file.read_to_end(&mut buf){
                Ok(_) => {},
                Err(err) => {
                    warn!("Error while reading file {:?}",err);
                    let err = IOError::from_error(err);
                    Self::write(f.id,FileForm::Error(err));
                    if let Some(ref t) = f.park{
                        t.unpark();
                    }
                    return;
                }
            };
            Self::write(f.id,FileForm::Raw(buf));
            if let Some(ref t) = f.park{
                t.unpark();
            }
        }
    }

    fn load_str(f: LoadData){
        if let Some(mut file) = Self::get_file(&f){
            let mut buf = String::new();
            match file.read_to_string(&mut buf){
                Ok(_) => {},
                Err(err) => {
                    error!("Error while reading file {:?}",err);
                    let err = IOError::from_error(err);
                    Self::write(f.id,FileForm::Error(err));
                    if let Some(ref t) = f.park{
                        t.unpark();
                    }
                    return;
                }
            };
            Self::write(f.id,FileForm::Str(buf));
            if let Some(ref t) = f.park{
                t.unpark();
            }
        }
    }

    fn get_file(f: &LoadData) -> Option<File>{
        let file = match File::open(f.path.clone()){
            Ok(file) => file,
            Err(e) => {
                let err = IOError::from_error(e);
                Self::write(f.id.clone(),FileForm::Error(err));
                if let Some(ref t) = f.park{
                    t.unpark();
                }
                return None;
            }
        };
        let metadata = match file.metadata(){
            Ok(meta) => meta,
            Err(e) => {
                let err = IOError::from_error(e);
                Self::write(f.id.clone(),FileForm::Error(err));
                if let Some(ref t) = f.park{
                    t.unpark();
                }
                return None;
            }
        };
        if metadata.is_dir(){
            Self::write(f.id.clone(),FileForm::Error(IOError::NotAFile));
            if let Some(ref t) = f.park{
                t.unpark();
            }
            return None;
        }
        return Some(file);
    }

    fn write(file_id: FileId,file :FileForm){
        FILES.write().expect("File lock poisend!").insert(file_id.inner(),Arc::new(file));
    }
}


#[derive(Debug)]
pub enum StreamCommand{
    Load(LoadData),
    LoadStr(LoadData),
    Quit,
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


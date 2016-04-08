use std::thread;
use std::thread::JoinHandle;

use std::fs::File;

use std::path::PathBuf;

use std::io::Read;

use super::FileId;
use super::IOError;

use std::sync::mpsc::{
    Receiver,
    Sender,
    channel,
    TryRecvError,
};

struct FileStream;

impl FileStream {
    fn run(reciever: Receiver<StreamCommand>,sender:Sender<StreamMessage>){
        for e in reciever{
            match e{
                StreamCommand::Quit => {break},
                StreamCommand::Load(file_id,path) => Self::load(file_id,path,&sender),
                StreamCommand::LoadStr(file_id,path) => Self::load_str(file_id,path,&sender),
                //_ => unimplemented!(),
            }
        }
    }

    fn load(file_id: FileId,path: PathBuf,sender: &Sender<StreamMessage>){
        if let Some(mut file) = Self::get_file(&file_id,path,sender){
            let mut buf = Vec::new();
            match file.read_to_end(&mut buf){
                Ok(_) => {},
                Err(err) => {
                    error!("Error while reading file {:?}",err);
                    sender.send(StreamMessage::Error(file_id,IOError::from_error(err))).unwrap();
                    return;
                }
            };
            sender.send(StreamMessage::Done(file_id,buf)).unwrap();
        }
    }

    fn load_str(file_id: FileId,path: PathBuf,sender: &Sender<StreamMessage>){
        if let Some(mut file) = Self::get_file(&file_id,path,sender){
            let mut buf = String::new();
            match file.read_to_string(&mut buf){
                Ok(_) => {},
                Err(err) => {
                    error!("Error while reading file {:?}",err);
                    sender.send(StreamMessage::Error(file_id,IOError::from_error(err))).unwrap();
                    return;
                }
            };
            sender.send(StreamMessage::DoneStr(file_id,buf)).unwrap();
        }
    }

    fn get_file(file_id: &FileId,path: PathBuf,sender: &Sender<StreamMessage>) -> Option<File>{
        let file = match File::open(path){
            Ok(file) => file,
            Err(e) => {
                sender.send(StreamMessage::Error(file_id.clone(),IOError::from_error(e))).unwrap();
                return None;
            }
        };
        let metadata = match file.metadata(){
            Ok(meta) => meta,
            Err(e) => {
                sender.send(StreamMessage::Error(file_id.clone(),IOError::from_error(e))).unwrap();
                return None;
            }
        };
        if metadata.is_dir(){
            sender.send(StreamMessage::Error(file_id.clone(),IOError::NotAFile)).unwrap();
            return None;
        }
        return Some(file);
    }
}



pub enum StreamCommand{
    Load(FileId,PathBuf),
    LoadStr(FileId,PathBuf),
    Quit,
}

pub enum StreamMessage{
    Done(FileId,Vec<u8>),
    DoneStr(FileId,String),
    Error(FileId,IOError),
}

pub struct StreamManager{
    thread: Option<JoinHandle<()>>,
    sender: Sender<StreamCommand>,
    reciever: Receiver<StreamMessage>,
}

impl StreamManager{

    pub fn new() -> Self{
        let (com_send,com_recv) = channel();
        let (mess_send,mess_recv) = channel();
        StreamManager{
            thread: Some(thread::spawn(||{
                FileStream::run(com_recv,mess_send);
            })),
            sender: com_send,
            reciever: mess_recv,
        }
    }

    pub fn get(&self) -> Option<StreamMessage>{
        match self.reciever.try_recv(){
            Ok(x) => {Some(x)},
            Err(e) => {
                match e{
                    TryRecvError::Empty => {None},
                    TryRecvError::Disconnected => {panic!("IO thread disconnected while recieving!");},
                }
            }
        }
    }

    pub fn send(&self,sc: StreamCommand){
        self.sender.send(sc).expect("IO thread disconnected while sending!");
    }
}

impl Drop for StreamManager{
    fn drop(&mut self){
        self.sender.send(StreamCommand::Quit).unwrap();
        self.thread.take().unwrap().join().unwrap();
    }
}

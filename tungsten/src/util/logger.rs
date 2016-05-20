use std::path::PathBuf;
use std::path::Path;

use std::io::Write;

use std::fs::File;

use std::sync::Mutex;

use log::{ 
        LogLevel,
        SetLoggerError,
        LogLevelFilter,
        LogMetadata,
        Log,
        LogRecord,
        self,
};

lazy_static!{
    static ref LOG_FILE_PATH: PathBuf 
        = Path::new("log.txt").to_path_buf();
    static ref LOG_FILE: Mutex<File> = {
        let mut file = File::create("log.txt").unwrap();
        writeln!(&mut file,"Log file from: {}"
                 ,time::now().strftime("%c").unwrap()).unwrap();
        Mutex::new(file)
    };
}

use time;

pub struct Logger;

impl Log for Logger{
    fn enabled(&self,meta: &LogMetadata) -> bool{
        meta.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord){
        if self.enabled(record.metadata()){
            let test = format!("[{}]{}: {}\n"
                     ,record.level()
                     ,time::now().strftime("%X").unwrap()
                     ,record.args());
            LOG_FILE.lock().unwrap().write_all(test.as_bytes()).unwrap();;
            print!("{}",test);
        }
    }
}

impl Logger{

    pub fn init() -> Result<(), SetLoggerError>{
        log::set_logger(|max_log_level|{
            max_log_level.set(LogLevelFilter::Info);
            Box::new(Logger)
        })
    }
}

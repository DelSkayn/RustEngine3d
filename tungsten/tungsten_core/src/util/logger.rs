use std::path::PathBuf;
use std::path::Path;

use std::io::Write;

use std::fs::{File,OpenOptions};
use std::sync::Mutex;

use log::{self, LogLevel, SetLoggerError, LogLevelFilter, LogMetadata, Log, LogRecord};

//use std::panic::{self,PanicInfo};

lazy_static!{
    static ref LOG_FILE_PATH: PathBuf 
        = Path::new("log.txt").to_path_buf();

    static ref LOG_FILE: Mutex<File> = {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&*LOG_FILE_PATH)
            .unwrap();
        writeln!(&mut file,"\nLog file from: {}"
                 ,time::now().strftime("%c").unwrap()).unwrap();
        Mutex::new(file)
    };
}

use time;

/// the basic logger.
/// Logs to a file specified by `LOG_FILE_PATH`;
/// TODO: Add option to change log file.
pub struct Logger;

impl Log for Logger {
    fn enabled(&self, meta: &LogMetadata) -> bool {
        meta.level() <= LogLevel::Trace
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let test = format!("[{}]{}: {}\n",
                               record.level(),
                               time::now().strftime("%X").unwrap(),
                               record.args());
            LOG_FILE.lock().unwrap().write_all(test.as_bytes()).unwrap();;
            print!("{}", test);
        }
    }
}

impl Logger {
    /// Initializes the logger
    /// Called once at the start of the engine.
    pub fn init() -> Result<(), SetLoggerError> {
        let res = log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Trace);
            Box::new(Logger)
        });
        // Disabled because it does not give a stack trace.
        //panic::set_hook(Box::new(log_panic));
        res
    }
}

/*
fn log_panic(info: &PanicInfo){
    if let Some(x) = info.location(){
        error!("Panic in file \"{}\" at line \"{}\"!",x.file(),x.line());
    }else{
        error!("Panic from unkown location!");
    }
    if let Some(x) = info.payload().downcast_ref::<&'static str>(){
        error!("Payload: \n{}",x);
    }else if let Some(x) = info.payload().downcast_ref::<String>(){
        error!("Payload: \n{}",x);
    }

}
*/

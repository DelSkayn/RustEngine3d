use std::path::PathBuf;
use std::path::Path;
use std::panic::{self,PanicInfo};

use std::io::Write;

use std::fs::{File,OpenOptions};
use std::sync::Mutex;

use registery::Registery;

use log::{self, LogLevel, SetLoggerError, LogLevelFilter, LogMetadata, Log, LogRecord};
extern crate backtrace;

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
pub struct Logger{
    level: LogLevel,
}

impl Log for Logger {
    fn enabled(&self, meta: &LogMetadata) -> bool {
        meta.level() <= self.level
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
            let level_string: String = Registery::get("log.level").or("debug".to_string());
            println!("Logging at level: {}",level_string);
            let level = match level_string.as_str(){
                "debug" => LogLevel::Debug,
                "error" => LogLevel::Error,
                "warn" => LogLevel::Warn,
                "info" => LogLevel::Info,
                "trace" => LogLevel::Trace,
                _ => unreachable!(),
            };
            if cfg!(debug_assertions){
                max_log_level.set(LogLevelFilter::Trace);
            }else{
                max_log_level.set(LogLevelFilter::Warn);
            }
            Box::new(Logger{
                level: level,
            })
        });
        // Disabled because it does not give a stack trace.
        panic::set_hook(Box::new(log_panic));
        res
    }
}

fn log_panic(info: &PanicInfo){
    if let Some(x) = info.location(){
        error!("Panic in file \"{}\" at line \"{}\"!",x.file(),x.line());
    }else{
        error!("Panic from unkown location!");
    }
    error!("    ##Backtrace##");
    backtrace::trace(|frame|{
        let ip = frame.ip();
        backtrace::resolve(ip,|sym|{
            let name = sym.name().map(|x| x.as_str().unwrap_or("! Error !")).unwrap_or("! Unkown !");
            let line = sym.lineno().map(|x| x as i64).unwrap_or(-1);
            let filename = sym.filename().map(|x| x.to_str().unwrap_or("! Error !")).unwrap_or("! Unkown !");
            error!("[backtrace] --> \x1b[31m{}\x1b[0m\n            file {}:{}",name,filename,line);
        });
        true
    });

}

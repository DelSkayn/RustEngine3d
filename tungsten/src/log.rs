use super::log_ext::{LogLevel,LogRecord, LogMetadata,SetLoggerError,LogLevelFilter};

use super::time;

pub struct SimpleLogger;

impl SimpleLogger{
    pub fn init() -> Result<(), SetLoggerError>{
        super::log_ext::set_logger(move |max_log_level| {
            //set the logging level based on wether were compiling
            //in release of debug
            if cfg!(debug_assertions){
                max_log_level.set(LogLevelFilter::Trace);
            }else{
                max_log_level.set(LogLevelFilter::Info);
            }
            Box::new(SimpleLogger)
        })
    }
} 

impl super::log_ext::Log for SimpleLogger{
    fn enabled(&self, metadata: &LogMetadata) -> bool{
        if cfg!(debug_assertions){
            metadata.level() <= LogLevel::Debug
        }else{
            metadata.level() <= LogLevel::Info
        }
    }
    fn log(&self, record: &LogRecord){
        if self.enabled(record.metadata()){
            if record.metadata().level() <= LogLevel::Debug{
                println!("[{}]|{}| {}",time::now().strftime("%T").unwrap()
                         ,record.level()
                         ,record.args());
            }else{
                println!("|{}| {}",record.level()
                         ,record.args());
            }
        }
    }
}


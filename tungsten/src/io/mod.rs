use std::path::PathBuf;

use std::thread;

mod stream;

struct FileId(u64);

struct IOSystem{
    stream: StreamManager,
}

impl IOSystem{
    fn new() -> Self{
        IOSystem{
            stream: StreamManager::new();
        }
    }
}

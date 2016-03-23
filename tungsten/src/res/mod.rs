
use super::System;
use super::kernal::JobBuilder;
use super::util::AtomicOption;
use super::Root;

trait Resource{}

struct FileId(u64);

struct ResourceData{
    queue: AtomicOption<Vec<FileId>>,
}

pub struct ResourcesSystem;

impl System for ResourcesSystem{
    fn run(&mut self,_root: &Root) -> Option<JobBuilder>{
        None
    }
}

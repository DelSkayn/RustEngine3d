
use super::System;
use super::kernel::TaskBuilder;
use super::util::AtomicOption;
use super::Root;

trait Resource{}

struct FileId(u64);

struct ResourceData{
    queue: AtomicOption<Vec<FileId>>,
}

pub struct ResourcesSystem;

impl System for ResourcesSystem{
    fn run(&mut self,_root: &Root) -> Option<TaskBuilder>{
        None
    }
}


use super::System;
use super::Schedular;
use super::AtomicOption;
use super::Root;

trait Resource{}

struct FileId(u64);

struct ResourceData{
    queue: AtomicOption<Vec<FileId>>,
}

pub struct ResourcesSystem;

impl System for ResourcesSystem{
    fn run(&mut self,root: &Root,schedular: &mut Schedular){
    }
}

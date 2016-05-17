extern crate context;

use self::context::Context;
use self::context::Transfer;
use self::context::stack::ProtectedFixedSizeStack;

use std::ptr;

struct StackManager{
    current: ProtectedFixedSizeStack,
    blocked: Vec<ProtectedFixedSizeStack>,
}

impl StackManager{
    fn new() -> Self{
        StackManager{
            current: ProtectedFixedSizeStack::new(),
            blocked: Vec::new(),
        }
    }

    fn run(Box<FnBox
}

fn run(worker: Worker<Box<Task>>,stealer: Vec<Stealer<Box<Task>>>){
    loop{
        if let Some(task) = worker.try_pop(){

        }else{
        }
    }
}

pub struct TaskManager{
    transfer: Transfer,
}

struct TaskGroup<'a>{
    task_manager: &'a TaskManger,
    tasks: Vec<Fn<()> + 'a>,
}

impl TaskManager{
    fn new(ts: Transfer){
        TaskManger{
            tasks: Vec::new(),
            transfer: ts,
        }
    }
}

extern "C" fn stack_run(mut t: Transfer){
    let task;
    unsafe{
        task = &mut *(t.data as *mut Fn<TaskSync>);
    };
    let tm = TaskManger::new(t);
    task(tm);
}

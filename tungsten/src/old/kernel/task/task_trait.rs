
pub enum TaskError{
    Failed,
    Reset,
}

pub type TaskResult = Result<Vec<TaskStruct>, TaskError>;

pub trait Task: Send + Sync{
    fn execute(&mut self) -> TaskResult;
}

#[derive(Clone)]
struct TaskSync{
    internal: Arc<AtomicIsize>,
}

impl TaskSync{
    fn new() -> Self{
        TaskSync{
            internal: Arc::new(AtomicIsize::new(0)),
        }
    }

    fn add_task(&self){
        internal.fetch_add(1,Ordering::Release);
    }

    fn task_done(&self){
        internal.fetch_sub(1,Ordering::Release);
    }

    fn is_done(&self) -> bool{
        internal.load(Ordering::Release) == 0
    }
}

pub struct TaskStruct{
    task: Box<Task>,
    dependencies: Vec<TaskSync>,
    sync: Option<TaskSync>,
}

impl TaskStruct{
    pub fn new<T: Task>(t: T) -> Self{
        TaskStruct{
            task: t,
            dependencies: Vec::new(),
            sync: None,
        }
    }
}

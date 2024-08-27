use crate::error::Result;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadId {
    Main,
    UI,
    Javascript,
}

impl std::fmt::Display for ThreadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreadId::Main => f.write_str("Main"),
            ThreadId::UI => f.write_str("UI"),
            ThreadId::Javascript => f.write_str("Javascript"),
        }
    }
}

type ThreadCallback = Box<dyn FnOnce() -> Result<()> + 'static + Send>;

pub struct ThreadRunner {
    thread_id: ThreadId,
    ///任务队列
    tasks: Arc<Mutex<Vec<ThreadCallback>>>,
}

impl ThreadRunner {
    pub fn new(thread_id: ThreadId) -> Self {
        ThreadRunner {
            thread_id,
            tasks: Default::default(),
        }
    }
}

impl ThreadRunner {
    pub fn post_task<FN>(&self, func: FN)
    where
        FN: FnOnce() -> Result<()> + 'static + Send,
    {
        self.tasks.lock().unwrap().push(Box::new(func));
    }

    pub fn run_once(&self) {
        let mut temp_tasks = Vec::<ThreadCallback>::new();
        if let Ok(mut tasks) = self.tasks.lock() {
            std::mem::swap(tasks.as_mut(), &mut temp_tasks);
        }

        for task in temp_tasks {
            if let Err(err) = task() {
                log::error!("{}: thread task run failed: {}", self.thread_id, err);
            }
        }
    }
}

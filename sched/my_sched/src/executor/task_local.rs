#![allow(unused)]

use std::pin::Pin;

use crate::executor::task_id::TaskId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Init,
    Pending,
    Running,
    Completed,
    Failed,
}

pub type TaskResult<T> = Result<T, String>; // Customize your error type as needed
pub(super) type TaskFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

pub(super) struct TaskLocal<T> {
    id: TaskId,
    name: String,
    state: TaskStatus,
    future: TaskFuture<T>,
}

impl<T> TaskLocal<T> {
    pub fn new<F>(id: TaskId, name: String, fut: F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        Self {
            id,
            state: TaskStatus::Init,
            name,
            future: Box::pin(fut),
        }
    }

    pub async fn run(self) -> T {
        self.future.await
    }
}

pub fn run_task() {
    println!("Task running!");
}

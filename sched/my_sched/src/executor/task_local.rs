#![allow(unused)]

use std::{
    cell::{Cell, RefCell},
    pin::Pin,
    task::{Context, Poll},
};

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
    state: Cell<TaskStatus>,
    future: RefCell<TaskFuture<T>>,
}

impl<T> TaskLocal<T> {
    pub fn new<F>(id: TaskId, name: String, fut: F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        Self {
            id,
            state: Cell::new(TaskStatus::Init),
            name,
            future: RefCell::new(Box::pin(fut)),
        }
    }

    pub fn set_status(&self, st: TaskStatus) {
        self.state.set(st)
    }

    /// Polls the future in-place without needing &mut self or Option wrapping
    pub fn poll(&self, cx: &mut Context<'_>) -> Poll<T> {
        self.future.borrow_mut().as_mut().poll(cx)
    }
    // pub fn take_future(&mut self) -> &mut TaskFuture<T> {
    //     self.future.borrow_mut().as_mut()
    // }

    // pub async fn run(self) -> T {
    //     self.future.await
    // }
}

pub fn run_task() {
    println!("Task running!");
}

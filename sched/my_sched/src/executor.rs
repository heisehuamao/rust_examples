#![allow(unused)]
mod task_handle;
pub mod task_id;
mod task_local;
mod task_manager;
mod worker;
// src/executor.rs

use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::executor::{task_id::TaskId, task_local::TaskStatus, task_manager::TaskManager};

/// Generates a lightweight, single-threaded dummy Waker for polling futures.
fn local_waker() -> Waker {
    fn noop(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        raw_waker()
    }

    // Static lifetime reference to the VTable
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);

    fn raw_waker() -> RawWaker {
        // RawWaker::new requires:
        // 1. data pointer (*const ())
        // 2. static reference to vtable (&'static RawWakerVTable)
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    unsafe { Waker::from_raw(raw_waker()) }
}

pub struct Executor {
    manager: TaskManager,
}

impl Executor {
    // pub fn new() -> Self {
    //     Self {
    //         manager: TaskManager::new(),
    //     }
    // }

    // pub fn manager_mut(&mut self) -> &mut TaskManager {
    //     &mut self.manager
    // }

    pub fn spawn<F>(&mut self, name: &str, fut: F) -> TaskId
    where
        F: Future<Output = ()> + 'static + Send,
    {
        self.manager.spawn(name, fut)
    }

    /// Runs all scheduled tasks to completion in a cooperative single-threaded loop
    pub fn run(&mut self) {
        let waker = local_waker();
        let mut cx = Context::from_waker(&waker);

        // Keep running while there are task handles queued up
        while let Some(handle) = self.manager.pop_next() {
            let id = handle.id();

            // Fetch the actual TaskLocal from the table
            if let Some(task) = self.manager.get_task(id) {
                // Update state to Running
                task.set_status(TaskStatus::Running);

                // Take the future out of Cell/RefCell to poll it
                // if let Some(mut fut) = task.take_future() {
                match task.poll(&mut cx) {
                    Poll::Ready(()) => {
                        // Task completed: mark status and clean up from manager table
                        task.set_status(TaskStatus::Completed);
                        self.manager.remove_task(id);
                    }
                    Poll::Pending => {
                        // Task yielded: put future back into TaskLocal and requeue handle
                        task.set_status(TaskStatus::Pending);
                        self.manager.requeue(handle);
                    }
                }
                // }
            }
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            manager: TaskManager::new(),
        }
    }
}

// Submodule function example
pub fn run_default() {
    println!("Running default executor...");
}

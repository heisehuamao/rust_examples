#![allow(unused)]

use std::collections::{HashMap, VecDeque};

use crate::executor::{
    task_handle::{self, TaskHandle},
    task_id::TaskId,
    task_local::TaskLocal,
};

pub(super) struct TaskManager {
    next_id: u64,
    /// Task table: Maps TaskId to the shared LocalTaskHandle
    tasks: HashMap<TaskId, TaskLocal<()>>,
    /// Run queue: Stores task handles ready to be executed
    run_queue: VecDeque<TaskHandle>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            tasks: HashMap::new(),
            run_queue: VecDeque::new(),
        }
    }

    /// Spawns a new task, adding it to both the task table and the run-queue
    pub fn spawn<F>(&mut self, name: &str, fut: F) -> TaskId
    where
        F: Future<Output = ()> + 'static + Send,
    {
        let id = TaskId::new(self.next_id);
        self.next_id += 1;

        let task = TaskLocal::new(id, name.to_string(), fut);

        // Store in the table for status queries / management
        match !self.tasks.contains_key(&id) {
            true => {
                self.tasks.insert(id, task);
                let task_handle = TaskHandle::new(id);
                self.run_queue.push_back(task_handle);
            }
            false => println!("task creation failed"),
        }

        id
    }

    /// Pops the next task handle from the run-queue
    pub(super) fn pop_next(&mut self) -> Option<TaskHandle> {
        self.run_queue.pop_front()
    }

    /// Re-enqueues a handle back into the run queue (e.g. if pending/yielded)
    pub(super) fn requeue(&mut self, handle: TaskHandle) {
        self.run_queue.push_back(handle);
    }

    /// Retrieves a task handle by ID from the table
    pub(super) fn get_task(&self, id: TaskId) -> Option<&TaskLocal<()>> {
        self.tasks.get(&id)
    }

    /// Removes a task from the table once finished
    pub(super) fn remove_task(&mut self, id: TaskId) -> Option<TaskLocal<()>> {
        self.tasks.remove(&id)
    }

    pub(super) fn active_task_count(&self) -> usize {
        self.tasks.len()
    }

    pub(super) fn queue_len(&self) -> usize {
        self.run_queue.len()
    }
}

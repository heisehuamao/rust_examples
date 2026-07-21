#![allow(unused)]
use std::sync::Arc;

use crate::executor::{task_id::TaskId, task_local::TaskLocal};

pub(super) struct TaskHandle {
    id: TaskId,
}

impl TaskHandle {
    pub fn new(id: TaskId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> TaskId {
        self.id
    }
}

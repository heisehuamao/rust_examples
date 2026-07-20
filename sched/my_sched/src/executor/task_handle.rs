#![allow(unused)]
use std::sync::Arc;

use crate::executor::{task_id::TaskId, task_local::TaskLocal};

pub(super) struct TaskHandle {
    id: TaskId,
}

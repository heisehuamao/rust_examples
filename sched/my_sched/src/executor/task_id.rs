#![allow(unused)]

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct TaskId {
    pub id: u32,
}

impl TaskId {
    /// Generates a new TaskId wrapper
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

    /// Unwraps the inner primitive value
    pub const fn as_u32(self) -> u32 {
        self.id
    }
}

// Display implementation for friendly printing (e.g., "Task #1")
impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task #{}", self.id)
    }
}

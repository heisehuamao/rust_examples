#![allow(unused)]

use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct TaskId {
    pub id: u64,
}

impl PartialEq for TaskId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for TaskId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl TaskId {
    /// Generates a new TaskId wrapper
    pub const fn new(id: u64) -> Self {
        Self { id }
    }

    /// Unwraps the inner primitive value
    pub const fn as_u32(self) -> u64 {
        self.id
    }
}

// Display implementation for friendly printing (e.g., "Task #1")
impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task #{}", self.id)
    }
}

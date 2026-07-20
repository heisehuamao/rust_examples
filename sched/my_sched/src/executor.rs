pub mod task_handle;
pub mod task_id;
pub mod task_local;
pub mod worker;
// src/executor.rs

pub struct Executor {
    pub name: String,
}

impl Executor {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn execute(&self) {
        println!("Executing task with {}", self.name);
    }
}

// Submodule function example
pub fn run_default() {
    println!("Running default executor...");
}

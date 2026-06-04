use clap::{Args, Parser, Subcommand};
// 1. Define the main CLI structure
#[derive(Parser)]
#[command(name = "taskmgr")]
#[command(about = "A simple task manager CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// 2. Define the available subcommands using an enum
#[derive(Subcommand)]
enum Commands {
    /// Add a new task to the list
    Add(AddArgs),
    
    /// Remove a task
    Remove {
        /// The ID of the task to remove
        #[arg(short, long)]
        id: u32,

        /// Force removal without confirmation
        #[arg(short, long)]
        force: bool,
    },
}

// 3. You can define arguments inside a separate struct for cleaner code
#[derive(Args)]
struct AddArgs {
    /// The description of the task
    task_name: String,
}

fn main() {
    // Parse the data from the command line arguments
    let cli = Cli::parse();

    // Match against the parsed subcommand
    match &cli.command {
        Commands::Add(args) => {
            println!("Adding task: '{}'", args.task_name);
        }
        Commands::Remove { id, force } => {
            if *force {
                println!("Forcefully removing task ID: {}", id);
            } else {
                println!("Removing task ID: {}", id);
            }
        }
    }
}

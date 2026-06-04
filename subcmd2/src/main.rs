use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

// Define the log levels for the --log-level arg
#[derive(ValueEnum, Clone, Debug)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// 1. Root-level CLI structure with metadata and global flags
#[derive(Parser, Debug)]
#[command(name = "subcmd2")]
#[command(author = "lzl")]
#[command(version)]
#[command(about = "The command test code.", long_about = None)]
struct Cli {
    /*
     Here is how you can update the full code to include those global, root-level arguments (config_dir, log_level, verbose) alongside the metadata attributes (author, version, about).

    By using global = true on the root arguments, clap ensures these flags can be passed either before the subcommand or after it (e.g., zeroclaw --verbose add "Task" or zeroclaw add "Task" --verbose).
    */
    /// Path to the configuration directory
    #[arg(long, global = true)]
    config_dir: Option<PathBuf>,

    #[arg(long, global = true, value_enum)]
    log_level: Option<LogLevel>,

    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
    // command: Commands,
}

// 2. Your subcommands remain neatly separated here
#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task to the list
    Add(AddArgs),

    /// Remove a task
    Remove {
        /// The ID of the task to remove
        #[arg(short, long)]
        id: u32,
    },
}

#[derive(Args, Debug)]
struct AddArgs {
    /// The description of the task
    task_name: String,
}

fn main() {
    let a = LogLevel::Trace;
    println!("to to_possible_value: {:?}", a.to_possible_value());
    println!("value value_variants: {:?}", LogLevel::value_variants());

    let cli = Cli::parse();

    // You can access global flags regardless of which subcommand was called
    println!("--- Global Flag States ---");
    println!("Config Directory: {:?}", cli.config_dir);
    println!("Log Level: {:?}", cli.log_level);
    println!("Verbose Mode: {}", cli.verbose);
    println!("-------------------------\n");

    // Handle the specific subcommands
    if let Some(cmd) = &cli.command {
        match cmd {
            Commands::Add(args) => {
                println!("Executing 'add' with task: '{}'", args.task_name);
            }
            Commands::Remove { id } => {
                println!("Executing 'remove' for task ID: {}", id);
            }
        }
    }
}

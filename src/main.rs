mod task;
use clap::{Parser, Subcommand, Args};
#[derive (Parser)]
#[command(name="citrine", version="0.1.0")]
#[command(author="syntaxbullet <syntaxbullet@protonmail.com>")]
#[command(about="A simple task manager following the unix philosophy")]

pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive (Subcommand)]

enum Commands {
    Add(Add),
    Update(Update),
    Delete(Delete),
    List(List),
}

#[derive (Args)]
/// Add a new task to the task list
struct Add {
    /// The title of the task
    title: String,
    /// The due date of the task in rfc3339 format (e.g. 2021-01-01T00:00:00+00:00) 
    #[arg(short = 'd', long = "due")]
    due_date: Option<String>,
    /// The priority of the task [0-9] 
    #[arg(short = 'p', long = "priority")]
    priority: Option<u8>,
    /// The tags of the task , must be a comma separated list
    #[arg(short = 't', long = "tags")]
    tags: Option<Vec<String>>,
}
#[derive (Args)]
/// Update an existing task in the task list
struct Update {
    /// The id of the task to update
    id: u32,
    /// The due date of the task in rfc3339 format (e.g. 2021-01-01T00:00:00+00:00)
    #[arg(short = 'd', long = "due")]
    due_date: Option<String>,
    /// The priority of the task (0-9)
    #[arg(short = 'p', long = "priority")]
    priority: Option<u8>,
    /// The tags of the task, must be a comma separated list
    #[arg(short = 't', long = "tags")]
    tags: Option<Vec<String>>,
    /// The status of the task
    #[arg(short = 's', long = "status")]
    status: Option<String>,
    /// The title of the task
    #[arg(short = 'm', long = "message")]
    title: Option<String>,
    /// remove tags from the task 
    #[arg(short = 'r', long = "remove-tags")]
    remove_tag: Option<String>,
    /// append tags to the task 
    #[arg(short = 'a', long = "append-tags")]
    append_tag: Option<String>,
}

#[derive (Args)]
/// Delete an existing task from the task list
struct Delete {
    /// The id of the task to remove
    id: u32,
}

#[derive (Args)]
/// List all tasks in the task list filtered by the given options
struct List {
    /// The due date of the task in rfc3339 format (e.g. 2021-01-01T00:00:00+00:00) 
    #[arg(short = 'd', long = "due")]
    due_date: Option<String>,
    /// The priority of the task [0-9] 
    #[arg(short = 'p', long = "priority")]
    priority: Option<u8>,
    /// The tags of the task , must be a comma separated list
    #[arg(short = 't', long = "tags")]
    tags: Option<Vec<String>>,
    /// The status of the task
    #[arg(short = 's', long = "status")]
    status: Option<String>,
    /// The title of the task
    #[arg(short = 'm', long = "message")]
    title: Option<String>,
}
fn main() {
  let _cli = Cli::parse();
} 
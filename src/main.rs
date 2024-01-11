mod storage;
mod task;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "citrine", version = "0.1.0")]
#[command(author = "syntaxbullet <syntaxbullet@protonmail.com>")]
#[command(about = "A simple task manager following the unix philosophy")]

pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]

enum Commands {
    Add(Add),
    Update(Update),
    Delete(Delete),
    List(List),
}

#[derive(Args)]
/// Add a new task to the task list
struct Add {
    /// The title of the task
    title: String,
    /// The due date of the task in rfc3339 format (e.g. 2021-01-01T00:00:00+00:00) or naive-date format (e.g. 2021-01-01)
    #[arg(short = 'd', long = "due")]
    due_date: Option<String>,
    /// The priority of the task [0-9]
    #[arg(short = 'p', long = "priority")]
    priority: Option<u8>,
    /// The tags of the task , must be a comma separated list
    #[arg(short = 't', long = "tags")]
    tags: Option<Vec<String>>,
}
#[derive(Args)]
/// Update an existing task in the task list
struct Update {
    /// The id of the task to update
    id: u8,
    /// The due date of the task in rfc3339 format (e.g. 2021-01-01T00:00:00+00:00) or naive-date format (e.g. 2021-01-01)
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

#[derive(Args)]
/// Delete an existing task from the task list
struct Delete {
    /// The id of the task to remove
    id: u8,
}

#[derive(Args)]
/// List all tasks in the task list filtered by the given options
struct List {}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add(add)) => {
            let last_id = storage::get_last_id().unwrap_or(0);
            let task = task::Task {
                id: last_id + 1,
                title: add.title,
                status: ' ',
                due_date: add.due_date,
                priority: add.priority,
                tags: add.tags.unwrap_or(Vec::new()),
            };
            // validate the task
            if task.validate().is_err() {
                // if the task is invalid, print the error and exit
                let error = task.validate().unwrap_err();
                println!("{}", error);
                std::process::exit(1);
            }
            // append the task to the task list
            storage::append_task(task).unwrap();
        }
        Some(Commands::Update(update)) => {
            // check if the id of the task exists
            if storage::read_task(update.id).is_err() {
                println!("Task with id {} does not exist", update.id);
                std::process::exit(1);
            }
            // get the task to update
            let mut task = storage::read_task(update.id).unwrap();
            // update the task
            if let Some(due_date) = update.due_date {
                task.due_date = Some(due_date);
            }
            if let Some(priority) = update.priority {
                task.priority = Some(priority);
            }
            if let Some(status) = update.status {
                task.status = status.chars().next().unwrap();
            }
            if let Some(title) = update.title {
                task.title = title;
            }
            if let Some(append_tag) = update.append_tag {
                // append the tag list to the task, deduplicating the tags
                task.tags.append(
                    &mut append_tag
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>(),
                );
            }
            if let Some(remove_tag) = update.remove_tag {
                // remove the tag list from the task, deduplicating the tags
                task.tags.retain(|tag| {
                    !remove_tag
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>()
                        .contains(tag)
                });
            }
            // update the task in the task list
            // validate the task
            if task.validate().is_err() {
                // if the task is invalid, print the error and exit
                let error = task.validate().unwrap_err();
                println!("{}", error);
                std::process::exit(1);
            }
            storage::update_task(update.id, task).unwrap();
        }
        Some(Commands::Delete(delete)) => {
            // delete the task from the task list
            storage::delete_task(delete.id).unwrap();
        }
        Some(Commands::List(_list)) => {
            // get all tasks from the task list
            let tasks = storage::read_tasks().unwrap();
            // print all tasks
            for task in tasks {
                println!("{}", task);
            }
        }
        None => {
            println!("No command given");
        }
    }
}

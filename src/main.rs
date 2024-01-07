mod task;
use clap::{Args, Parser, Subcommand};

use crate::task::Task;
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
#[derive(Args)]
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

#[derive(Args)]
/// Delete an existing task from the task list
struct Delete {
    /// The id of the task to remove
    id: u32,
}

#[derive(Args)]
/// List all tasks in the task list filtered by the given options
struct List {}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add(add)) => {
            handle_add_command(add);
        }
        Some(Commands::Update(update)) => {
            handle_update_commmand(update);
        }
        Some(Commands::Delete(delete)) => {
            handle_delete_command(delete);
        }
        Some(Commands::List(list)) => handle_list_command(list),
        None => {
            println!("No command given");
        }
    }
}

fn handle_add_command(add: Add) {
    // ensure the .citrine file exists
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(".citrine")
        .unwrap();

    let last_id = task::Task::last_id();
    let task = task::Task {
        id: last_id + 1,
        title: add.title,
        due_date: add.due_date,
        priority: add.priority,
        tags: add.tags,
        status: task::Status::OPEN,
    };
    if task.title.is_empty() {
        println!("Title must not be empty");
        return;
    }
    if let Some(due_date) = task.due_date.clone() {
        match chrono::DateTime::parse_from_rfc3339(&due_date) {
            Ok(_) => {
                let now = chrono::Utc::now();
                let due_date = chrono::DateTime::parse_from_rfc3339(&due_date).unwrap();
                if due_date < now {
                    println!("Due date must be in the future");
                    return;
                }
            }
            Err(_) => {
                println!("Due date must be in rfc3339 format");
                return;
            }
        }
    }
    if let Some(priority) = task.priority {
        if priority > 9 {
            println!("Priority must be between 0 and 9");
            return;
        }
    }
    println!("{:?}", Task::stringify(task.clone()));
    task::Task::append_to_file(task)
}
fn handle_update_commmand(update: Update) {
    let new_task = task::Task::find_in_file(update.id);
    // if we haven't found a task with the given id, return
    if new_task.is_none() {
        println!("No task with id {} found", update.id);
        return;
    }
    let mut new_task = new_task.unwrap();
    if let Some(due_date) = update.due_date {
        match chrono::DateTime::parse_from_rfc3339(&due_date) {
            Ok(_) => {
                let now = chrono::Utc::now();
                let due_date = chrono::DateTime::parse_from_rfc3339(&due_date).unwrap();
                if due_date < now {
                    println!("Due date must be in the future");
                    return;
                }
            }
            Err(_) => {
                println!("Due date must be in rfc3339 format");
                return;
            }
        }
        new_task.due_date = Some(due_date);
    }
    if let Some(priority) = update.priority {
        if priority > 9 {
            println!("Priority must be between 0 and 9");
            return;
        }
        new_task.priority = Some(priority);
    }
    if let Some(tags) = update.tags {
        new_task.tags = Some(tags);
    }
    if let Some(status) = update.status {
        match status.as_str() {
            "open" => new_task.status = task::Status::OPEN,
            "in-progress" => new_task.status = task::Status::INPROGRESS,
            "done" => new_task.status = task::Status::DONE,
            "overdue" => new_task.status = task::Status::OVERDUE,
            _ => {
                println!("Status must be one of open, in-progress, done, or overdue");
                return;
            }
        }
    }
    if let Some(title) = update.title {
        new_task.title = title;
    }
    if let Some(remove_tag) = update.remove_tag {
        let mut tags = new_task.tags.unwrap();
        let remove_tag = remove_tag.split(",").collect::<Vec<&str>>();
        for tag in remove_tag {
            if tags.contains(&tag.to_string()) {
                tags.remove(tags.iter().position(|x| x == &tag.to_string()).unwrap());
            }
        }
        new_task.tags = Some(tags);
    }
    if let Some(append_tag) = update.append_tag {
        let mut tags = new_task.tags.unwrap();
        let append_tag = append_tag.split(",").collect::<Vec<&str>>();
        for tag in append_tag {
            if !tags.contains(&tag.to_string()) {
                tags.push(tag.to_string());
            }
        }
        new_task.tags = Some(tags);
    }
    println!("{:?}", Task::stringify(new_task.clone()));
    task::Task::update_in_file(new_task, update.id);
}
fn handle_delete_command(delete: Delete) {
    task::Task::remove_from_file(delete.id);
}
fn handle_list_command(_list: List) {
    // open the .citrine file and read the contents
    let file_contents = std::fs::read_to_string(".citrine").unwrap();
    let contents = file_contents.lines().collect::<Vec<&str>>();
    // loop through each line and print it out
    contents.iter().for_each(|line| {
        println!("{}", line);
    });
}

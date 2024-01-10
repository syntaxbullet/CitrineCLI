
use std::error::Error;
use std::fs::{File, OpenOptions};

use crate::task::Task;
use std::io::Write;
use std::io::Read;

pub fn append_task(task: Task) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(".citrine")?;
    // validate the task
    if !task.validate() {
        println!("Invalid task");
        return Ok(());
    }
    writeln!(file, "{}", task)?;
    Ok(())
}
pub fn read_task(id: u8) -> Result<Task, Box<dyn Error>> {
    let mut file = File::open(".citrine")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for line in contents.lines() {
        let task = Task::from_string(line.to_string()).unwrap();
        if task.id == id {
            return Ok(task);
        }
    }
    Err("Task not found".into())
}
pub fn read_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let mut file = File::open(".citrine")?;
    // if the file is empty, return an empty vector
    if file.metadata()?.len() == 0 {
        return Ok(Vec::new());
    }
    // if the file does not exist, return an empty vector
    if !std::path::Path::new(".citrine").exists() {
        return Ok(Vec::new());
    }
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut tasks = Vec::new();
    for line in contents.lines() {
        // skip unparseable lines
        if let Some(task) = Task::from_string(line.to_string()) {
            tasks.push(task);
        }
    }
    Ok(tasks)
}
pub fn write_tasks(tasks: Vec<Task>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(".citrine")?;
    for task in tasks {
        writeln!(file, "{}", task)?;
    }
    Ok(())
}
pub fn delete_task(id: u8) -> Result<(), Box<dyn Error>> {
    let mut tasks = read_tasks()?;
    tasks.retain(|task| task.id != id);
    write_tasks(tasks)?;
    Ok(())
}
pub fn update_task(id: u8, task: Task) -> Result<(), Box<dyn Error>> {
    let mut tasks = read_tasks()?;
    tasks.retain(|t| t.id != id);
    tasks.push(task);
    write_tasks(tasks)?;
    Ok(())
}
pub fn get_last_id() -> Result<u8, Box<dyn Error>> {
    let tasks = read_tasks()?;
    let mut last_id = 0;
    for task in tasks {
        if task.id > last_id {
            last_id = task.id;
        }
    }
    Ok(last_id)
}

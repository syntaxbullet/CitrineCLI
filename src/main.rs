mod task;
use task::Task;
fn main() {
    // 1. [x] some task - due: 2020-02-20T12:00:00+09:00 priority: 1 tags: tag1,tag2
    let some_task_string = "1. [>] some task - tags: tag1,tag2 due: 2020-02-20T12:00:00+09:00 priority: 1";
    let task = Task::parse(some_task_string).unwrap();
    println!("{:?}", Task::stringify(task));
} 
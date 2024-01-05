use task::parse_task_from_string;

mod task;

fn main() {
    // 1. [x] some task - due: 2020-02-20T12:00:00+09:00 priority: 1 tags: tag1,tag2
    let some_task_string = "1. [>] some task - due: 2020-02-20T12:00:00+09:00 priority: 1 tags: tag1,tag2";
    let task: task::Task = parse_task_from_string(some_task_string).unwrap();
    println!("{:?}", task);
}

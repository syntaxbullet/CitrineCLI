mod task;
fn main() {
    let t = task::Task::from_string("1. [>] \"Task 1\"; due: 2020-01-21T00:00; priority: 1; tags: tag1, tag2".to_string()).unwrap();
    let is_valid = t.validate();
    println!("{}", is_valid);
    println!("{}", t);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: Status,
    pub due_date: Option<String>,
    pub priority: Option<u8>,
    pub tags: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Done,
    Overdue,
}
fn parse_from_string(input: &str) -> Option<Task> {
    // 1. [x] some task - due: 2020-02-20T12:00:00+09:00 priority: 1 tags: tag1,tag2
    let mut due_date: Option<String> = None;
    let mut priority: Option<u8> = None;
    let mut tags: Option<Vec<String>> = None;

    let status: Status;

    // to optain the id, we need to parse all characters until the first dot, and parse into u32
    let id: u32 = input
        .chars()
        .take_while(|c| c != &'.')
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    // obtaining the status is easy, we just need to look for an [ and a ] and grab that, then we match it to a status depending on which char is inside
    let openbracket = input.chars().position(|c| c == '[').unwrap();
    // check if there is a closing bracket exactly 2 chars after the opening bracket
    let closedbracket = input.chars().position(|c| c == ']').unwrap();
    if closedbracket - openbracket == 2 {
        status = match input.chars().nth(openbracket + 1).unwrap() {
            'x' => Status::Done,
            ' ' => Status::Open,
            '>' => Status::InProgress,
            '!' => Status::Overdue,
            _ => panic!("Failed to parse Task: Invalid status"),
        };
    } else {
        panic!("Failed to parse Task: Invalid status");
    };
    // obtaining the title is a bit more tricky, we need to find the first space after the closing bracket, and then take all characters until we encounter a dash or the string ends
    let title: String = input
        .chars()
        .skip(closedbracket + 1)
        .take_while(|c| c != &'-')
        .collect::<String>()
        .trim()
        .to_string();

    let due_prefix_index: Option<usize> = input.find("due: ");
    let priority_prefix_index: Option<usize> = input.find("priority: ");
    let tags_prefix_index: Option<usize> = input.find("tags: ");
    // we want to skip the due: prefix, so we need to add 5 to the index and then use chars().skip().take_while()
    if due_prefix_index.is_some() {
        due_date = Some(
            input
                .chars()
                .skip(due_prefix_index.unwrap() + 5)
                .take_while(|c| c != &' ')
                .collect::<String>(),
        );
    }
    if priority_prefix_index.is_some() {
        priority = Some(
            input
                .chars()
                .skip(priority_prefix_index.unwrap() + 10)
                .take_while(|c| c != &' ')
                .collect::<String>()
                .parse::<u8>()
                .unwrap(),
        );
    }
    if tags_prefix_index.is_some() {
        tags = Some(
            input
                .chars()
                .skip(tags_prefix_index.unwrap() + 6)
                .take_while(|c| c != &' ')
                .collect::<String>()
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }
    Some(Task {
        id,
        title,
        status,
        due_date,
        priority,
        tags,
    })
}

fn stringify(task: Task) -> String {
    let mut output = String::new();
    output.push_str(&task.id.to_string());
    output.push_str(". [");
    output.push_str(match task.status {
        Status::Open => " ",
        Status::InProgress => ">",
        Status::Done => "x",
        Status::Overdue => "!",
    });
    output.push_str("] ");
    output.push_str(&task.title);
    if task.due_date.is_some() {
        output.push_str(" - due: ");
        output.push_str(&task.due_date.unwrap());
    }
    if task.priority.is_some() {
        output.push_str(" - priority: ");
        output.push_str(&task.priority.unwrap().to_string());
    }
    if task.tags.is_some() {
        output.push_str(" - tags: ");
        output.push_str(&task.tags.unwrap().join(","));
    }
    output
}
impl Task {
    pub fn parse(input: &str) -> Option<Task> {
        parse_from_string(input)
    }
    pub fn stringify(task: Task) -> String {
        stringify(task)
    }
    pub fn find_in_file(id: u32) -> Option<Task> {
        // opens the .citrine file and grabs all lines, then filters them by the id via Task::parse() if the id matches we return the task
        let file = std::fs::read_to_string(".citrine");
        if file.is_err() {
            panic!("Failed to open file: {}", file.unwrap_err());
        }
        let file = file.unwrap();
        let lines = file.lines();
        for line in lines {
            let task = Task::parse(line);
            if task.is_some() && task.as_ref().unwrap().id == id {
                return task.clone();
            }
        }
        None
    }
    pub fn append_to_file(task: Task) {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(".citrine")
            .unwrap();
        std::io::Write::write(&mut file, format!("{}\n", Task::stringify(task)).as_bytes())
            .unwrap();
    }
    pub fn remove_from_file(id: u32) {
        let file = std::fs::read_to_string(".citrine");
        if file.is_err() {
            panic!("Failed to open file: {}", file.unwrap_err());
        }
        let file = file.unwrap();
        let lines = file.lines();
        let mut output = String::new();
        for line in lines {
            let task = Task::parse(line);
            if task.is_some() && task.as_ref().unwrap().id != id {
                output.push_str(&format!("{}\n", line));
            }
        }
        std::fs::write(".citrine", output).unwrap();
    }
    pub fn update_in_file(new_task: Task, id: u32) {
        let file = std::fs::read_to_string(".citrine");
        if file.is_err() {
            panic!("Failed to open file: {}", file.unwrap_err());
        }
        let file = file.unwrap();
        let lines = file.lines();
        let mut output = String::new();
        for line in lines {
            let task = Task::parse(line);
            if task.is_some() {
                if task.as_ref().unwrap().id != id {
                    output.push_str(&format!("{}\n", line));
                } else {
                    output.push_str(&format!("{}\n", Task::stringify(new_task.clone())));
                }
            }
        }
        std::fs::write(".citrine", output).unwrap();
    }
    pub fn last_id() -> u32 {
        let file = std::fs::read_to_string(".citrine");
        if file.is_err() {
            panic!("Failed to open file: {}", file.unwrap_err());
        }
        let file = file.unwrap();
        let lines = file.lines();
        let mut last_id = 0;
        for line in lines {
            let task = Task::parse(line);
            if task.is_some() && task.as_ref().unwrap().id > last_id {
                last_id = task.as_ref().unwrap().id;
            }
        }
        last_id
    }
}

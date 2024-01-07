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
    OPEN,
    INPROGRESS,
    DONE,
    OVERDUE,
}
fn parse_from_string(input: &str) -> Option<Task> {
    // 1. [x] some task - due: 2020-02-20T12:00:00+09:00 priority: 1 tags: tag1,tag2
    let mut due_date: Option<String> = None;
    let mut priority: Option<u8> = None;
    let mut tags: Option<Vec<String>> = None;
    let id: u32;
    let status: Status;
    let title: String;

    // to optain the id, we need to parse all characters until the first dot, and parse into u32
    id = input
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
            'x' => Status::DONE,
            ' ' => Status::OPEN,
            '>' => Status::INPROGRESS,
            '!' => Status::OVERDUE,
            _ => panic!("Failed to parse Task: Invalid status"),
        };
    } else {
        panic!("Failed to parse Task: Invalid status");
    }
    // obtaining the title is a bit more tricky, we need to find the first space after the closing bracket, and then take all characters until we encounter a dash or the string ends
    title = input
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
                .split(",")
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
        Status::OPEN => " ",
        Status::INPROGRESS => ">",
        Status::DONE => "x",
        Status::OVERDUE => "!",
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
    return output;
}
impl Task {
    pub fn parse(input: &str) -> Option<Task> {
        parse_from_string(input)
    }
    pub fn stringify(task: Task) -> String {
        stringify(task)
    }
    // pub fn get_tags(&self) -> Option<Vec<String>> {
    //     self.tags.clone()
    // }

    // pub fn set_id(&mut self, id: u32) {
    //     self.id = id;
    // }
    // pub fn set_title(&mut self, title: String) {
    //     self.title = title;
    // }

    // pub fn set_status(&mut self, status: Status) {
    //     // do not allow setting the status to OVERDUE if there is no due date from the past
    //     if status == Status::OVERDUE && self.due_date.is_none() {
    //         panic!("Failed to set status: Cannot set status to OVERDUE if there is no due date");
    //     }
    //     // check if the due date is in the past, if it is not and the status is OVERDUE, return an error message
    //     if status == Status::OVERDUE && self.due_date.is_some() {
    //         let parsed_date = chrono::DateTime::parse_from_rfc3339(&self.due_date.clone().unwrap());
    //         if parsed_date.is_err() {
    //             panic!("Failed to parse due date: {}", parsed_date.unwrap_err());
    //         }
    //         let parsed_date = parsed_date.unwrap();
    //         if parsed_date > chrono::Utc::now() {
    //             panic!("Failed to set status: Cannot set status to OVERDUE if the due date is in the future");
    //         }
    //     }
    //     self.status = status;
    // }
    // pub fn set_due_date(&mut self, due_date: Option<String>) {
    //     // attempt to parse the date, if it fails, return an error, if it is in the past, return an error
    //     if due_date.is_some() {
    //         let parsed_date = chrono::DateTime::parse_from_rfc3339(&due_date.clone().unwrap());
    //         if parsed_date.is_err() {
    //             panic!("Failed to parse due date: {}", parsed_date.unwrap_err());
    //         }
    //         let parsed_date = parsed_date.unwrap();
    //         if parsed_date < chrono::Utc::now() {
    //             panic!("Failed to set due date: Date is in the past");
    //         }
    //     }

    //     self.due_date = due_date;
    // }
    // pub fn set_priority(&mut self, priority: Option<u8>) {
    //     // if priority is set, it must be between 1 and 9
    //     if priority.is_some() {
    //         if priority.unwrap() < 1 || priority.unwrap() > 9 {
    //             panic!("Failed to set priority: Priority must be between 1 and 9");
    //         }
    //     }
    //     self.priority = priority;
    // }
    // pub fn set_tags(&mut self, tags: Option<Vec<String>>) {
    //     // if tags are set, they must be a vector of strings that are not empty and do not contain spaces
    //     if tags.is_some() {
    //         for tag in tags.clone().unwrap() {
    //             if tag.is_empty() || tag.contains(" ") {
    //                 panic!(
    //                     "Failed to set tags: Tags must not be empty and must not contain spaces"
    //                 );
    //             }
    //         }
    //     }
    //     self.tags = tags;
    // }
    // pub fn remove_tag(&mut self, tag: String) {
    //     if self.tags.is_some() {
    //         let mut tags = self.tags.clone().unwrap();
    //         if tags.contains(&tag) {
    //             tags.remove(tags.iter().position(|t| t == &tag).unwrap());
    //             self.tags = Some(tags);
    //         } else {
    //             panic!("Failed to remove tag: Tag does not exist");
    //         }
    //     } else {
    //         panic!("Failed to remove tag: Tag does not exist");
    //     }
    // }
    // pub fn append_tag(&mut self, tag: String) {
    //     if self.tags.is_some() {
    //         let mut tags = self.tags.clone().unwrap();
    //         if !tags.contains(&tag) {
    //             tags.push(tag);
    //             self.tags = Some(tags);
    //         } else {
    //             panic!("Failed to add tag: Tag already exists");
    //         }
    //     } else {
    //         let mut tags = self.tags.clone().unwrap_or(Vec::new());
    //         tags.push(tag);
    //         self.tags = Some(tags);
    //     }
    // }
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
            if task.is_some() {
                if task.as_ref().unwrap().id == id {
                    return task.clone();
                }
            }
        }
        return None;
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
            if task.is_some() {
                if task.as_ref().unwrap().id != id {
                    output.push_str(&format!("{}\n", line));
                }
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
            if task.is_some() {
                if task.as_ref().unwrap().id > last_id {
                    last_id = task.as_ref().unwrap().id;
                }
            }
        }
        return last_id;
    }
}

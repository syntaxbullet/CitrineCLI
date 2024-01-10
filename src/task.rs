pub struct Task {
    pub id: u8,
    pub title: String,
    pub status: char, // " ", "x",">","!"
    pub due_date: Option<String>,
    pub priority: Option<u8>,
    pub tags: Vec<String>,
}
impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("id: {}\n", self.id));
        output.push_str(&format!("title: {}\n", self.title));
        output.push_str(&format!("status: {}\n", self.status));
        if let Some(due_date) = &self.due_date {
            output.push_str(&format!("due_date: {}\n", due_date));
        }
        if let Some(priority) = self.priority {
            output.push_str(&format!("priority: {}\n", priority));
        }
        if !self.tags.is_empty() {
            output.push_str(&format!("tags: {}\n", self.tags.join(", ")));
        }
        write!(f, "{}", output)
    }
}
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("{}. ", self.id));
        output.push_str(&format!("[{}] ", self.status));
        output.push_str(&format!("\"{}\"; ", self.title));

        if let Some(due_date) = &self.due_date {
            output.push_str(&format!("due: {}; ", due_date));
        }
        if let Some(priority) = self.priority {
            output.push_str(&format!("priority: {}; ", priority));
        }
        if !self.tags.is_empty() {
            output.push_str(&format!("tags: {}; ", self.tags.join(", ")));
        }
        write!(f, "{}", output)
    }
}

impl Task {
    pub fn from_string(input: String) -> Option<Task> {
        let mut result = Task {
            id: 0,
            title: String::new(),
            status: ' ',
            due_date: None,
            priority: None,
            tags: Vec::new(),
        };

        result.id = input.split('.').next()?.parse::<u8>().unwrap();
        result.status = input.split('[').nth(1)?.chars().next()?;
        result.title = input.split('"').nth(1)?.to_string();

        let mut attributes = input.split(';');
        attributes.next();

        for attribute in attributes {
            let mut attribute = attribute.split(':');
            let key = attribute.next()?.trim();
            let value = attribute.next()?.trim();

            match key {
                "due" => result.due_date = Some(value.to_string()),
                "priority" => result.priority = Some(value.parse::<u8>().unwrap()),
                "tags" => result.tags = value.split(',').map(|s| s.trim().to_string()).collect(),
                _ => (),
            }
        }
        // validate the task
        if !result.validate() {
            return None;
        }
        Some(result)
    }
    pub fn validate(&self) -> bool {
        if self.id == 0 {
            return false;
        }
        if self.title.is_empty() {
            return false;
        }
        if self.status != ' ' && self.status != 'x' && self.status != '>' && self.status != '!' {
            return false;
        }
        if let Some(priority) = self.priority {
            if priority < 1 || priority > 9 {
                return false;
            }
        }
        if let Some(due_date) = &self.due_date {
            // if the date can't be parsed as rfc3339, or naive, then return false
            if chrono::DateTime::parse_from_rfc3339(due_date).is_err()
                && chrono::NaiveDate::parse_from_str(due_date, "%Y-%m-%d").is_err()
            {
                return false;
            }
            if chrono::DateTime::parse_from_rfc3339(due_date).is_ok() {
                let date = chrono::DateTime::parse_from_rfc3339(due_date).unwrap();
                if date < chrono::Utc::now() {
                    return false;
                }
            }
        }
        true
    }
}

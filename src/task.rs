use std::error::Error;

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
    pub fn from_string(input: String) -> Result<Task, Box<dyn Error>>{
        let mut result = Task {
            id: 0,
            title: String::new(),
            status: ' ',
            due_date: None,
            priority: None,
            tags: Vec::new(),
        };
        result.id = input.split('.').next().unwrap().parse::<u8>().unwrap_or(0);
        result.status = input.split('[').nth(1).unwrap().chars().next().unwrap_or('-');
        result.title = input.split('"').nth(1).unwrap_or("").to_string();

        let mut attributes = input.split(';');
        attributes.next();

        for attribute in attributes {
            let mut attribute = attribute.split(':');
            let key = attribute.next().unwrap().trim();
            let value = attribute.next().unwrap_or("").trim();

            match key {
                "due" => result.due_date = Some(value.to_string()),
                "priority" => result.priority = Some(value.parse::<u8>().unwrap()),
                "tags" => result.tags = value.split(',').map(|s| s.trim().to_string()).collect(),
                _ => (),
            }
        }
        // validate the task result and error if it is invalid
        result.validate()?;
        Ok(result)
    }
    pub fn validate(&self) -> Result<bool, Box<dyn Error>> {
        if self.id == 0 {
            return Err("Invalid ID 0".into());
        }
        if self.title.is_empty() {
            return Err("Invalid title".into());
        }
        if self.status != ' ' && self.status != 'x' && self.status != '>' && self.status != '!' {
            return Err("Invalid status".into());
        }
        if let Some(priority) = self.priority {
            if priority < 1 || priority > 9 {
                return Err("Invalid priority".into());
            }
        }
        if let Some(due_date) = &self.due_date {
            // if the date can't be parsed as rfc3339, or naive, then return false
            if chrono::DateTime::parse_from_rfc3339(due_date).is_err()
                && chrono::NaiveDate::parse_from_str(due_date, "%Y-%m-%d").is_err()
            {
                return Err("Invalid date".into());
            }
            if chrono::DateTime::parse_from_rfc3339(due_date).is_ok() {
                let date = chrono::DateTime::parse_from_rfc3339(due_date).unwrap();
                if date < chrono::Utc::now() {
                    return Err("Invalid date".into());
                }
            }
        }
        Ok(true)
    }
}

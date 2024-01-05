pub struct Task {
    id: u32,
    title: String,
    status: Status,
    due_date: Option<String>,
    priority: Option<u8>,
    tags: Option<Vec<String>>,
}

enum Status {
    OPEN,
    INPROGRESS,
    DONE,
    OVERDUE,
    
}

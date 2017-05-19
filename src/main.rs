extern crate chrono;

use chrono::{DateTime, UTC};

fn main() {
    println!("Hello World");
    let mut root = Task::new();
}

enum Status {
    Started { date: DateTime<UTC> },
    Followup { date: DateTime<UTC> },
    Completed { date: DateTime<UTC> }
}

struct Task {

    due_date: Option<DateTime<UTC>>,
    history: Vec<Status>
    
    task: String,
    comment: Option<String>,
    priority: i8,

    parent: Option<Task>,
    children: Vec<Task>
}

impl Task {
    fn new(parent: Option<Task>, task: String) -> Task {
        Task {
            due_date: None,
            history: vec![ Status::Started { date : UTC:now() } ],
            
            task: task, 
            comment: None
                priority: 1,
            
            parent: parent,
            children: None
        }     
    }
    fn change_status(
}

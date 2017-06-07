use chrono::{DateTime, UTC};

#[derive(Debug)]
enum Status {
    Started { date: DateTime<UTC> },
    Followup { date: DateTime<UTC> },
    Completed { date: DateTime<UTC> }
}

pub struct Task {

    due_date: Option<DateTime<UTC>>,
    history: Vec<Status>,
    
    task: String,
    comment: Option<String>,
    priority: i8,
}

impl Task {
    pub fn new(task: String) -> Task {
        Task {
            due_date: None,
            history: vec![ Status::Started { date : UTC::now() } ],
            
            task: task, 
            comment: None,
	    priority: 1,
        }     
    }
    
    pub fn print(&self) {
        println!("Task<{}>", self.task);
    }

    // STATUS RELATED FUNCTIONS

    pub fn get_status(&self) -> Option<&Status> {
        self.history.last()
    }

    pub fn start(&mut self) {
        self.history.push( Status::Started { date : UTC::now() } );
    }

    pub fn followup(&mut self) {
        self.history.push( Status::Followup { date : UTC::now() } );
    }

    pub fn complete(&mut self) {
        self.history.push( Status::Completed { date : UTC::now() } );
    }

    // change task
    pub fn set(&mut self, new_task : String) {
        self.task = new_task;
    }

}

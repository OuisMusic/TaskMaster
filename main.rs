use std::collections::HashMap;
use std::io::{self, Write};

struct Task {
    id: u32,
    title: String,
    priority: u8,
}

struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_task_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_task_id: 1,
        }
    }

    fn add_task(&mut self, title: String, priority: u8) {
        let new_task = Task {
            id: self.next_task_id,
            title,
            priority,
        };
        self.tasks.insert(self.next_task_id, new_task);
        self.next_task_id += 1;
    }

    fn list_all_tasks(&self) {
        for task in self.tasks.values() {
            println!("ID: {} | Title: {} | Priority: {}", task.id, task.title, task.priority);
        }
    }

    fn remove_task_by_id(&mut self, task_id: u32) {
        self.tasks.remove(&task_id);
    }
}

struct PriorityManager {}

impl PriorityManager {
    fn update_task_priority(task: &mut Task, new_priority: u8) {
        task.priority = new_priority;
    }
}

fn main() {
    let mut manager = TaskManager::new();

    loop {
        println!("TaskMaster Options:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Remove Task");
        println!("4. Exit");

        let mut user_choice = String::new();
        io::stdin().read_line(&mut user_choice).unwrap_or_else(|_| panic!("Failed to read input"));

        match user_choice.trim() {
            "1" => {
                println!("Task Title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap_or_else(|_| panic!("Failed to read title"));
                let title = title.trim().to_string();

                println!("Task Priority (1-5):");
                let mut priority_string = String::new();
                io::stdin().read_line(&mut priority_string).unwrap_or_else(|_| panic!("Failed to read priority"));
                let priority: u8 = priority_string.trim().parse().expect("Priority should be a number 1-5");

                manager.add_task(title, priority);
            },
            "2" => manager.list_all_tasks(),
            "3" => {
                println!("Task ID to Remove:");
                let mut id_string = String::new();
                io::stdin().read_line(&mut id_string).unwrap_or_else(|_| panic!("Failed to read ID"));
                let id: u32 = id_string.trim().parse().expect("ID should be a number");

                manager.remove_task_by_id(id);
            },
            "4" => break,
            _ => println!("Invalid Option, Try Again."),
        }
        
        println!("\n");
    }
}
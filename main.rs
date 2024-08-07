use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

struct Task {
    id: u32,
    title: String,
    priority: u8,
}

struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String, priority: u8) {
        let task = Task {
            id: self.next_id,
            title,
            priority,
        };
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    fn list_tasks(&self) {
        for task in self.tasks.values() {
            println!("{}: {} (Priority: {})", task.id, task.title, task.priority);
        }
    }

    fn remove_task(&mut self, id: u32) {
        self.tasks.remove(&id);
    }
}

struct PriorityManager {}

impl PriorityManager {
    fn adjust_priority(task: &mut Task, new_priority: u8) {
        task.priority = new_priority;
    }
}

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        println!("Please choose an option:");
        println!("1. Add a task");
        println!("2. List all tasks");
        println!("3. Remove a task");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = title.trim().to_string();

                println!("Enter task priority (1-5):");
                let mut priority_str = String::new();
                io::stdin().read_line(&mut priority_str).expect("Failed to read line");
                let priority: u8 = priority_str.trim().parse().expect("Please type a number!");

                task_manager.add_task(title, priority);
            },
            "2" => task_manager.list_tasks(),
            "3" => {
                println!("Enter the ID of the task to remove:");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                let id: u32 = id_str.trim().parse().expect("Please type a number!");

                task_manager.remove_task(id);
            },
            "4" => break,
            _ => println!("Invalid option, try again."),
        }

        println!("\n");
    }
}
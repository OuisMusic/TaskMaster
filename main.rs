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
        let task = Task {
            id: self.next_task_id,
            title,
            priority,
        };
        self.tasks.insert(self.next_task_id, task);
        self.next_task_id += 1;
    }

    fn display_all_tasks(&self) {
        for task in self.tasks.values() {
            println!("ID: {} | Title: {} | Priority: {}", task.id, task.title, task.priority);
        }
    }

    fn delete_task_by_id(&mut self, task_id: u32) {
        self.tasks.remove(&task_id);
    }
}

struct PriorityManager {}

impl PriorityManager {
    fn set_task_priority(task: &mut Task, new_priority: u8) {
        task.priority = new_priority;
    }
}

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        println!("TaskMaster Options:");
        println!("1. Add Task");
        println!("2. Display Tasks");
        println!("3. Delete Task");
        println!("4. Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap_or_else(|_| panic!("Failed to read input"));

        match option.trim() {
            "1" => {
                println!("Enter Task Title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap_or_else(|_| panic!("Failed to read title"));
                let title = title.trim().to_string();

                println!("Enter Task Priority (1-5):");
                let mut priority_input = String::new();
                io::stdin().read_line(&mut priority_input).expect("Failed to read priority");
                let priority: u8 = priority_input.trim().parse().expect("Priority should be a number between 1-5");

                task_manager.add_task(title, priority);
            },
            "2" => task_manager.display_all_tasks(),
            "3" => {
                println!("Enter Task ID to Delete:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read ID");
                let task_id: u32 = id_input.trim().parse().expect("ID should be a number");

                task_manager.delete_task_by_id(task_id);
            },
            "4" => break,
            _ => println!("Invalid option, please try again."),
        }
        
        println!("\n");
    }
}
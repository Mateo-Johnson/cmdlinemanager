use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        let tasks = Self::load_tasks();
        let next_id = if tasks.is_empty() { 1 } else { tasks.len() + 1 };
        TaskManager { tasks, next_id }
    }

    fn load_tasks() -> Vec<Task> {
        if !Path::new("tasks.json").exists() {
            return Vec::new();
        }
        let data = fs::read_to_string("tasks.json").expect("Unable to read file");
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    }

    fn save_tasks(&self) {
        let data = serde_json::to_string(&self.tasks).expect("Unable to serialize tasks");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("tasks.json")
            .expect("Unable to open file");
        file.write_all(data.as_bytes()).expect("Unable to write data");
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        self.save_tasks();
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            let status = if task.completed { "✓" } else { "✗" };
            println!("{} [ID: {}] {}", status, task.id, task.description);
        }
    }

    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
            self.save_tasks();
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn delete_task(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(pos);
            self.save_tasks();
        } else {
            println!("Task with ID {} not found.", id);
        }
    }
}

fn main() {
    let mut manager = TaskManager::new();
    loop {
        println!("Task Manager");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "1" => {
                let mut description = String::new();
                println!("Enter task description:");
                io::stdin().read_line(&mut description).expect("Failed to read line");
                manager.add_task(description.trim().to_string());
            }
            "2" => {
                manager.list_tasks();
            }
            "3" => {
                let mut id = String::new();
                println!("Enter task ID to complete:");
                io::stdin().read_line(&mut id).expect("Failed to read line");
                manager.complete_task(id.trim().parse().expect("Invalid ID"));
            }
            "4" => {
                let mut id = String::new();
                println!("Enter task ID to delete:");
                io::stdin().read_line(&mut id).expect("Failed to read line");
                manager.delete_task(id.trim().parse().expect("Invalid ID"));
            }
            "5" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

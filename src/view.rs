// view.rs
use std::io;
use crate::model::Task;

pub fn show_tasks(tasks: &Vec<Task>) {
    println!("To-Do List:");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { " [X]" } else { " [ ]" };
        println!("{}{} {}", index + 1, status, task.description);
    }
}

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
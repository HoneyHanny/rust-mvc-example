// main.rs
mod model;
mod view;
mod controller;

use crate::controller::Controller;

fn main() {
    let mut controller = Controller::new();

    loop {
        println!("1. Add Task");
        println!("2. Toggle Task");
        println!("3. Show Tasks");
        println!("4. Quit");

        let choice = view::read_input();

        match choice.as_str() {
            "1" => {
                println!("Enter task description:");
                let description = view::read_input();
                controller.add_task(description);
            }
            "2" => {
                controller.show_tasks();
                println!("Enter the task number to toggle:");
                if let Ok(index) = view::read_input().parse::<usize>() {
                    controller.toggle_task(index - 1);
                } else {
                    println!("Invalid input. Please enter a valid task number.");
                }
            }
            "3" => {
                controller.show_tasks();
            }
            "4" => {
                break;
            }
            _ => {
                println!("Invalid choice. Please select a valid option.");
            }
        }
    }
}
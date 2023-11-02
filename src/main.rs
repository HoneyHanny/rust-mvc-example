// main.rs
mod model;
mod view;
mod controller;

use cursive::Cursive;
use crate::controller::Controller;

fn main() {
    let mut cursive = Cursive::default();
    let mut controller = Controller::new();

    controller.add_task("Buy groceries".to_string());
    controller.add_task("Finish Rust project".to_string());

    controller.show_tasks(&mut cursive);

    cursive.run();
}
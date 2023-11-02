// view.rs
use cursive::views::{Dialog, ListView, TextView};
use cursive::Cursive;

pub fn show_tasks(cursive: &mut Cursive, tasks: &Vec<Task>) {
    let task_list = ListView::new()
        .with(|list| {
            for task in tasks {
                list.add_child("", TextView::new(task.description.clone()));
            }
        });

    cursive.add_layer(
        Dialog::new()
            .title("To-Do List")
            .content(task_list)
            .button("Quit", |s| s.quit()),
    );
}
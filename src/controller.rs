// controller.rs
use crate::model::{TaskList, Task};
use crate::view;

pub struct Controller {
    task_list: TaskList,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            task_list: TaskList::new(),
        }
    }

    pub fn add_task(&mut self, description: String) {
        self.task_list.add_task(description);
    }

    pub fn show_tasks(&self) {
        view::show_tasks(&self.task_list.get_tasks());
    }
}
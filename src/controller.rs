// controller.rs
use crate::model::TaskList;
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

    pub fn toggle_task(&mut self, index: usize) {
        if let Some(task) = self.task_list.tasks.get_mut(index) {
            task.completed = !task.completed;
        }
    }

    pub fn show_tasks(&self) {
        view::show_tasks(&self.task_list.get_tasks());
    }
}
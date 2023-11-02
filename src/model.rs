// model.rs
pub struct Task {
    pub description: String,
    pub completed: bool,
}

pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: vec![] }
    }

    pub fn add_task(&mut self, description: String) {
        self.tasks.push(Task {
            description,
            completed: false,
        });
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
use ratatui::widgets::ListState;
use time::Date;

use super::task::{Status, Task};

#[derive(Debug, Default)]
pub struct TaskList {
    pub task_list: Vec<Task>,
    pub state: ListState,
}

impl TaskList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_task(&mut self, title: String, description: Vec<String>, due_date: Date) {
        self.task_list.push(Task::new(title, description, due_date));
    }

    pub fn get_selected_task(&self) -> Option<&Task> {
        self.state.selected().map(|i| &self.task_list[i])
    }

    pub fn delete_selected_task(&mut self) {
        if let Some(i) = self.state.selected() {
            self.task_list.remove(i);
        }
    }

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.state.selected() {
            self.task_list[i].status = match self.task_list[i].status {
                Status::Todo => Status::Completed,
                Status::Completed => Status::Todo,
            }
        }
    }

    pub fn toggle_favorite(&mut self) {
        if let Some(i) = self.state.selected() {
            self.task_list[i].is_favorite = !self.task_list[i].is_favorite
        }
    }

    pub fn select_none(&mut self) {
        self.state.select(None);
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }
    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }
}

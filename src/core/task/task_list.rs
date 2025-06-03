use std::ops::IndexMut;

use ratatui::widgets::ListState;
use time::Date;
use uuid::Uuid;

use crate::{ui::components::task_list, utils::date::get_today_with_fallbacks};

use super::task::{Status, Task};

#[derive(Debug, Default)]
pub struct TaskList {
    pub task_list: Vec<Task>,
    pub state: ListState,
    pub selected_task_id: Option<Uuid>,
}

impl TaskList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_task(&mut self, title: String, description: Vec<String>, due_date: Date) {
        let new_task = Task::new(title, description, due_date);
        self.selected_task_id = Some(new_task.id);
        self.task_list.push(new_task);
        self.sync_selection_state();
    }

    pub fn get_selected_task(&self) -> Option<&Task> {
        self.selected_task_id
            .and_then(|id| self.task_list.iter().find(|task| task.id == id))
    }

    pub fn get_selected_task_mut(&mut self) -> Option<&mut Task> {
        if let Some(id) = self.selected_task_id {
            self.task_list.iter_mut().find(|task| task.id == id)
        } else {
            None
        }
    }

    pub fn get_sorted_tasks(&mut self) -> Vec<&Task> {
        use itertools::Itertools;

        self.task_list
            .iter()
            .sorted_by(|a, b| {
                let a_completed = a.status == Status::Completed;
                let b_completed = b.status == Status::Completed;

                if a_completed != b_completed {
                    return a_completed.cmp(&b_completed);
                }

                a.is_favorite.cmp(&b.is_favorite).reverse()
            })
            .collect()
    }

    pub fn get_selected_display_index(&mut self) -> Option<usize> {
        self.selected_task_id.and_then(|id| {
            self.get_sorted_tasks()
                .iter()
                .position(|task| task.id == id)
        })
    }

    // Use when we have self borrowing (only for internal)
    fn find_task_display_index(sorted_tasks: &[&Task], selected_id: Option<Uuid>) -> Option<usize> {
        selected_id.and_then(|id| sorted_tasks.iter().position(|task| task.id == id))
    }

    pub fn sync_selection_state(&mut self) {
        let index = self.get_selected_display_index();
        self.state.select(index);
    }

    pub fn delete_selected_task(&mut self) {
        if let Some(selected_id) = self.selected_task_id {
            let deleted_index = self.get_selected_display_index().unwrap_or(0);

            self.task_list.retain(|task| task.id != selected_id);
            self.selected_task_id = None;

            if !self.task_list.is_empty() {
                // Maintain the cursor position after delete
                self.select_task_near_deleted_task(deleted_index);
            }

            self.sync_selection_state();
        }
    }

    pub fn select_task_near_deleted_task(&mut self, deleted_index: usize) {
        let sorted_tasks = self.get_sorted_tasks();

        if sorted_tasks.is_empty() {
            return;
        }

        let target_index = deleted_index.min(sorted_tasks.len() - 1);

        if let Some(task) = sorted_tasks.get(target_index) {
            self.selected_task_id = Some(task.id);
        }
    }

    pub fn toggle_status(&mut self) {
        if let Some(task) = self.get_selected_task_mut() {
            match task.status {
                Status::Completed => {
                    task.status = Status::Todo;
                    task.completed_at = None;
                }
                Status::Todo => {
                    task.status = Status::Completed;
                    task.completed_at = Some(get_today_with_fallbacks().0);
                }
            }
        }

        self.sync_selection_state();
    }

    pub fn toggle_favorite(&mut self) {
        if let Some(task) = self.get_selected_task_mut() {
            task.is_favorite = !task.is_favorite
        }
        self.sync_selection_state();
    }

    pub fn select_none(&mut self) {
        self.selected_task_id = None;
        self.sync_selection_state();
    }

    pub fn select_next(&mut self) {
        let selected_id = self.selected_task_id;
        let tasks = self.get_sorted_tasks();

        if tasks.is_empty() {
            return;
        }

        let current_index = Self::find_task_display_index(&tasks, selected_id).unwrap_or(0);
        let next_index = (current_index + 1) % tasks.len();

        self.selected_task_id = Some(tasks[next_index].id);
        self.sync_selection_state();
    }

    pub fn select_previous(&mut self) {
        let selected_id = self.selected_task_id;
        let tasks = self.get_sorted_tasks();

        if tasks.is_empty() {
            return;
        }

        let current_index = Self::find_task_display_index(&tasks, selected_id).unwrap_or(0);
        let next_index = (current_index - 1) % tasks.len();

        self.selected_task_id = Some(tasks[next_index].id);
        self.sync_selection_state();
    }

    pub fn select_first(&mut self) {
        let tasks = self.get_sorted_tasks();

        if tasks.is_empty() {
            return;
        }

        self.selected_task_id = Some(tasks[0].id);
        self.sync_selection_state();
    }

    pub fn select_last(&mut self) {
        let tasks = self.get_sorted_tasks();

        if tasks.is_empty() {
            return;
        }

        self.selected_task_id = Some(tasks[tasks.len() - 1].id);
        self.sync_selection_state();
    }
}

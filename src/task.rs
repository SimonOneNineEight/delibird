use ratatui::{
    style::{
        Color, Modifier, Style,
        palette::{
            material::GREEN,
            tailwind::{SLATE, YELLOW},
        },
    },
    text::Line,
    widgets::{ListItem, ListState},
};
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

use crate::task_form::FormInput;

const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;
const STAR_TEXT_FG_COLOR: Color = YELLOW.c200;
pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Status {
    Todo,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: Uuid,
    pub title: String,
    pub description: Vec<String>,
    pub status: Status,
    // pub group: Option<String>,
    pub is_favorite: bool,
    pub due_date: Option<Date>,
    pub created_at: Date,
    pub completed_at: Option<Date>,
}

impl Task {
    pub fn new(task: FormInput) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: task.title.lines()[0].clone(),
            description: task.description.into_lines(),
            status: Status::Todo,
            is_favorite: false,
            // group: None,
            created_at: OffsetDateTime::now_local()
                .unwrap_or_else(|_| OffsetDateTime::now_utc())
                .date(),
            completed_at: None,
            due_date: Some(task.due_date.selected_date),
        }
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(value: &Task) -> Self {
        let line = match value.status {
            Status::Todo => {
                if value.is_favorite {
                    Line::styled(format!("  ✮ {}", value.title), STAR_TEXT_FG_COLOR)
                } else {
                    Line::styled(format!("  ☐ {}", value.title), TEXT_FG_COLOR)
                }
            }
            Status::Completed => {
                Line::styled(format!("  ✓ {}", value.title), COMPLETED_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}

#[derive(Debug, Default)]
pub struct TaskList {
    pub task_list: Vec<Task>,
    pub state: ListState,
}

impl TaskList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_task(&mut self, task: FormInput) {
        self.task_list.push(Task::new(task));
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

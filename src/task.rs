use chrono::{DateTime, Local};
use ratatui::{
    style::{
        Color, Modifier, Style,
        palette::{material::GREEN, tailwind::SLATE},
    },
    text::Line,
    widgets::{ListItem, ListState},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;
pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
enum Status {
    Todo,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: Uuid,
    description: String,
    status: Status,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            status: Status::Todo,
            created_at: Local::now(),
            completed_at: None,
        }
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(value: &Task) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!(" ☐ {}", value.description), TEXT_FG_COLOR),
            Status::Completed => {
                Line::styled(format!(" ✓ {}", value.description), COMPLETED_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}

#[derive(Debug)]
pub struct TaskList {
    pub task_list: Vec<Task>,
    pub state: ListState,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            task_list: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn add_task(&mut self, description: String) {
        self.task_list.push(Task::new(description));
    }

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.state.selected() {
            self.task_list[i].status = match self.task_list[i].status {
                Status::Todo => Status::Completed,
                Status::Completed => Status::Todo,
            }
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

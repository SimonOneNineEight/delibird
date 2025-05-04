use chrono::{DateTime, Local};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{
        Color, Modifier, Style,
        palette::{material::GREEN, tailwind::SLATE},
    },
    symbols,
    text::Line,
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, StatefulWidget, block::title,
    },
};
use uuid::Uuid;

const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;
pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Todo,
    Completed,
}

#[derive(Debug, Clone)]
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
    task_list: Vec<Task>,
    state: ListState,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            task_list: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        self.task_list.clone()
    }

    pub fn get_state(&self) -> ListState {
        self.state.clone()
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

use ratatui::{
    style::{
        Color, Modifier, Style,
        palette::{
            material::GRAY,
            tailwind::{SLATE, YELLOW},
        },
    },
    text::Line,
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use crate::utils::date::get_today_with_fallbacks;

const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Style = Style::new()
    .fg(GRAY.c500)
    .add_modifier(Modifier::CROSSED_OUT);
const STAR_TEXT_FG_COLOR: Color = YELLOW.c200;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Status {
    Todo,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Uuid,
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
    pub fn new(title: String, description: Vec<String>, due_date: Date) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            status: Status::Todo,
            is_favorite: false,
            // group: None,
            created_at: get_today_with_fallbacks().0,
            completed_at: None,
            due_date: Some(due_date),
        }
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(value: &Task) -> Self {
        let line = match value.status {
            Status::Todo => {
                if value.is_favorite {
                    Line::styled(format!(" ✮ {}", value.title), STAR_TEXT_FG_COLOR)
                } else {
                    Line::styled(format!(" ☐ {}", value.title), TEXT_FG_COLOR)
                }
            }
            Status::Completed => {
                Line::styled(format!(" ✓ {}", value.title), COMPLETED_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}

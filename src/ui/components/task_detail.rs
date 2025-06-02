use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use time::macros::format_description;

use crate::{app::App, core::task::task::Status};

impl App {
    pub fn render_detail(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("Task Detail")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let detail_area = block.inner(area);

        let [title_area, due_date_area, description_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .areas(detail_area);

        block.render(area, buf);

        if let Some(task) = self.task_list.get_selected_task() {
            let status = if task.status == Status::Todo {
                Line::from("Todo").style(
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::ITALIC),
                )
            } else {
                Line::from("Complete").style(
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::ITALIC),
                )
            };
            let title = vec![status, Line::from(task.title.clone())];
            let due_date = task
                .due_date
                .unwrap()
                .format(format_description!("[year]-[month]-[day]"))
                .unwrap_or_else(|_| "No Due Date".to_string());

            let description: Vec<Line> = task
                .description
                .iter()
                .map(|line| Line::from(line.as_str()))
                .collect();

            Paragraph::new(title)
                .block(Block::default())
                .style(Style::default().fg(Color::White))
                .render(title_area, buf);

            Paragraph::new(due_date)
                .block(Block::default())
                .style(Style::default().fg(Color::White))
                .render(due_date_area, buf);

            Paragraph::new(description)
                .block(Block::default())
                .style(Style::default().fg(Color::White))
                .render(description_area, buf);
        } else {
            Paragraph::new("No Task Selected!")
                .block(Block::default())
                .centered()
                .render(detail_area, buf)
        }
    }
}

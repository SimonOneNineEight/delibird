use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

use crate::{app::App, ui::get_center_rect};

impl App {
    pub fn render_delete_popup(&self, area: Rect, buf: &mut Buffer) {
        if !self.show_delete_popup {
            return;
        }
        if let Some(task) = self.task_list.get_selected_task() {
            let confirm_area = get_center_rect(32, 4, area);

            Clear.render(confirm_area, buf);

            let confirm_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Yellow).bg(Color::Black));

            let text = vec![
                Line::from(format!("Delete Task: {}?", task.title)),
                Line::from("[Y]es   [N]o"),
            ];

            Paragraph::new(text)
                .alignment(ratatui::layout::Alignment::Center)
                .block(confirm_block)
                .render(confirm_area, buf);
        }
    }
}

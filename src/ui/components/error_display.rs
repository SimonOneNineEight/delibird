use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

use crate::{
    app::App,
    core::error::ErrorSeverity,
    ui::{get_center_rect, get_warning_rect},
};

impl App {
    pub fn render_error_state(&mut self, area: Rect, buf: &mut Buffer) {
        if let Some(error) = &self.error_state.current_error {
            match error.severity() {
                ErrorSeverity::Warning => {
                    self.render_warning_banner(area, buf);
                }
                ErrorSeverity::Error => {
                    if self.error_state.should_show_popup() {
                        self.render_error_popup(area, buf);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn render_warning_banner(&mut self, area: Rect, buf: &mut Buffer) {
        if let Some(message) = self.error_state.current_message() {
            let warning_area = get_warning_rect(40, 3, area);

            Clear.render(warning_area, buf);

            Paragraph::new(message)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .style(Style::default().bg(Color::Black).fg(Color::Yellow)),
                )
                .style(Style::default().fg(Color::Yellow))
                .render(warning_area, buf);
        }
    }
    pub fn render_error_popup(&mut self, area: Rect, buf: &mut Buffer) {
        if let Some(message) = self.error_state.current_message() {
            let popup_area = get_center_rect(60, 8, area);

            Clear.render(popup_area, buf);

            let block = Block::default()
                .title("Error")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Red).bg(Color::Black));

            Paragraph::new(format!("{}\n\n\n\nPress <Esc> to dismiss", message))
                .block(block)
                .style(Style::default().fg(Color::White))
                .render(popup_area, buf);
        }
    }
}

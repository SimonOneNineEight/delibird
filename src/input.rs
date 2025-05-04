use crossterm::event::KeyEvent;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};
use tui_textarea::TextArea;

use crate::app::InputMode;

#[derive(Debug)]
pub struct InputField {
    textarea: TextArea<'static>,
}

impl Default for InputField {
    fn default() -> Self {
        let mut input_field = Self {
            textarea: TextArea::default(),
        };

        input_field.style_textarea(InputMode::Normal);

        input_field
    }
}

impl InputField {
    pub fn style_textarea(&mut self, input_mode: InputMode) {
        self.textarea.set_style(Style::default().fg(Color::White)); // Ensure text is visible

        // Then set the block with all necessary properties
        let border_style = match input_mode {
            InputMode::Editing => Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Color::LightCyan),
        };

        self.textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title("New Task"), // Add a title if needed
        );

        // Set cursor line style if needed
        self.textarea.set_cursor_line_style(Style::default());
    }

    pub fn textarea(&self) -> &TextArea<'static> {
        &self.textarea
    }

    pub fn textarea_mut(&mut self) -> &mut TextArea<'static> {
        &mut self.textarea
    }

    pub fn input(&mut self, key: KeyEvent) {
        self.textarea.input(key);
    }

    pub fn clear(&mut self) {
        self.textarea = TextArea::default();
        self.style_textarea(InputMode::Editing);
    }
}

use crate::{
    app::App,
    ui::{forms::task_form::FormField, get_center_rect},
};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Padding, Widget, block::Position},
};
use tui_textarea::TextArea;

impl App {
    pub fn render_add_task_popup(&mut self, area: Rect, buf: &mut Buffer) {
        let popup_area = get_center_rect(90, 20, area);

        Clear.render(popup_area, buf);

        let popup_block = Block::default()
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .padding(Padding::symmetric(1, 1))
            .title("Press <tab> to change focus, <C-Enter> to submit")
            .title_position(Position::Bottom)
            .title_alignment(Alignment::Center)
            .title_style(Style::default().fg(Color::LightCyan));

        let inner_area = popup_block.inner(popup_area);

        popup_block.render(popup_area, buf);

        let [title_area, due_date_area, description_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(inner_area);

        self.render_popup_form_textarea(title_area, buf, "Title".to_string(), FormField::Title);

        self.render_popup_form_textarea(
            description_area,
            buf,
            "Description".to_string(),
            FormField::Description,
        );
        self.render_popup_form_date(area, due_date_area, buf);
    }

    pub fn render_popup_form_textarea(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        title: String,
        field: FormField,
    ) {
        let border_style = self.task_form.get_input_border_style(field);
        let cursor_style = self.task_form.get_cursor_style(field);

        let block = Block::bordered()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(title)
            .border_style(border_style);

        let textarea = self
            .task_form
            .get_field::<TextArea<'static>>(field)
            .unwrap();

        textarea.set_block(block.clone());
        textarea.set_cursor_style(cursor_style);

        textarea.render(area, buf);
    }

    pub fn render_popup_form_date(&mut self, total_area: Rect, input_area: Rect, buf: &mut Buffer) {
        let border_style = self.task_form.get_input_border_style(FormField::DueDate);
        let cursor_style = self.task_form.get_cursor_style(FormField::DueDate);

        self.task_form.form_input.due_date.render(
            total_area,
            input_area,
            buf,
            border_style,
            cursor_style,
        );
    }
}

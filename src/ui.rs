use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, StatefulWidget,
        Widget,
        block::Position,
        calendar::{CalendarEventStore, Monthly},
    },
};
use time::macros::format_description;
use tui_textarea::TextArea;

use crate::{
    app::App,
    task::{SELECTED_STYLE, Status},
    task_form::FormField,
};

impl Widget for &mut App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let wrapper_block = Block::default()
            .padding(Padding::symmetric(2, 1))
            .style(Style::default().bg(Color::Black).fg(Color::LightCyan));

        let inner_area = wrapper_block.inner(area);

        wrapper_block.render(area, buf);

        let [main_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(2)]).areas(inner_area);

        let [list_area, detail_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area);

        self.render_task_list(list_area, buf);
        self.render_detail(detail_area, buf);
        self.render_footer(footer_area, buf);

        if self.task_form.is_open {
            self.render_add_task_popup(area, buf);
        }
    }
}

impl App {
    pub fn render_task_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("Task List")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        if self.task_list.task_list.is_empty() {
            Paragraph::new("Press n to add new task!")
                .block(block)
                .centered()
                .render(area, buf)
        } else {
            // Iterate through all elements in the `items` and stylize them.
            let items: Vec<ListItem> = self
                .task_list
                .task_list
                .iter()
                .sorted_by(|a, b| {
                    let a_completed = a.status == Status::Completed;
                    let b_completed = b.status == Status::Completed;

                    if a_completed != b_completed {
                        return a_completed.cmp(&b_completed);
                    }

                    a.is_favorite.cmp(&b.is_favorite).reverse()
                })
                .map(ListItem::from)
                .collect();

            let list = List::new(items)
                .block(block)
                .highlight_style(SELECTED_STYLE)
                .highlight_symbol(">>")
                .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

            StatefulWidget::render(list, area, buf, &mut self.task_list.state);
        }
    }

    fn render_detail(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("Task Detail")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let detail_area = block.inner(area);
        block.render(area, buf);

        if let Some(task) = self.task_list.get_selected_task() {
            let details = vec![Line::raw(format!("Description: \n {}", task.title))];

            Paragraph::new(details)
                .block(Block::default())
                .render(detail_area, buf);
        } else {
            Paragraph::new("No Task Selected!")
                .block(Block::default())
                .centered()
                .render(detail_area, buf)
        }
    }

    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        let full_text = "Add Task: n | Previous: k | Next: j | Toggle Complete: <space> | Delete Task: d | Cancel: <esc> | Quit: q".to_string();
        let short_text = "Add: n | Previous: k | Next: j | Toggle: <space> | Delete: d | Cancel: <esc> | Quit: q".to_string();
        let tiny_text = "Add:n | Toggle:<space> | Quit:q".to_string();

        // Get the width of the footer area
        let width = area.width;

        // Choose text based on available width
        let text = if width >= full_text.len() as u16 {
            full_text
        } else if width >= short_text.len() as u16 {
            short_text
        } else {
            tiny_text
        };

        let paragraph = Paragraph::new(text).centered();
        paragraph.render(area, buf);
    }

    fn render_add_task_popup(&mut self, area: Rect, buf: &mut Buffer) {
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

pub fn get_center_rect(width: u16, height: u16, area: Rect) -> Rect {
    let popup_width = width.min(area.width.saturating_sub(4));
    let popup_height = height.min(area.height.saturating_sub(4));

    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = (area.height.saturating_sub(popup_height)) / 2;

    Rect::new(
        area.x + popup_x,
        area.y + popup_y,
        popup_width,
        popup_height,
    )
}

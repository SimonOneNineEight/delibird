use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::{App, InputMode},
    task::SELECTED_STYLE,
};

impl Widget for &mut App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let wrapper_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black).fg(Color::LightCyan));

        let inner_area = wrapper_block.inner(area);

        wrapper_block.render(area, buf);

        let [list_area, input_area, footer_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .areas(inner_area);

        self.render_task_list(list_area, buf);
        self.input.textarea().render(input_area, buf);

        let text = format!("Press `Ctrl-C` or `q` to quit delibird. Press `n` to add task.",);

        let paragraph = Paragraph::new(text).centered();

        paragraph.render(footer_area, buf);
    }
}

impl App {
    pub fn render_task_list(&mut self, area: Rect, buf: &mut Buffer) {
        let border_style = match self.input_mode {
            InputMode::Normal => Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Color::LightCyan),
        };

        let block = Block::new()
            .title(Line::raw("Delibird").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .task_list
            .get_tasks()
            .iter()
            .enumerate()
            .map(|(i, todo_item)| ListItem::from(todo_item))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.

        StatefulWidget::render(list, area, buf, &mut self.task_list.get_state());
    }
}

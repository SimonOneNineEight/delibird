use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style, palette::tailwind::SLATE},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, StatefulWidget, Widget},
};

use itertools::Itertools;

use crate::{app::App, core::task::task::Status};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

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
                .render(area, buf);
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
}

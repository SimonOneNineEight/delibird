use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Padding, Widget},
};

use crate::app::App;

pub mod error_display;
pub mod footer;
pub mod messages;
pub mod popups;
pub mod task_detail;
pub mod task_list;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let wrapper_block = Block::default()
            .padding(Padding::symmetric(2, 1))
            .style(Style::default().bg(Color::Black).fg(Color::White));

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
        self.render_error_state(area, buf);

        if self.task_form.is_open {
            self.render_add_task_popup(area, buf);
        }
    }
}

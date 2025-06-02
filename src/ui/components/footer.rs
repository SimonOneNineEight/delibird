use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app::App;

impl App {
    pub fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        let full_text = "Add Task: n | Previous: k | Next: j | Toggle Complete: <Enter> | Delete Task: d | Cancel: <esc> | Quit: q".to_string();
        let short_text = "Add: n | Previous: k | Next: j | Toggle: <Enter> | Delete: d | Cancel: <esc> | Quit: q".to_string();
        let tiny_text = "Add:n | Toggle:<Enter> | Quit:q".to_string();

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

        Paragraph::new(text).centered().render(area, buf);
    }
}

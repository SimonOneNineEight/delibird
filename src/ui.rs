use ratatui::layout::Rect;

pub mod components;
pub mod forms;

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

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{
        Block, BorderType, Borders, Clear, Paragraph, Widget,
        calendar::{CalendarEventStore, Monthly},
    },
};
use time::{Date, Duration, Month, OffsetDateTime, macros::format_description};
use tui_textarea::TextArea;

use crate::{ui::get_center_rect, utils::date::get_today_with_fallbacks};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DateInputMode {
    Text,
    Calendar,
}

#[derive(Debug, Clone)]
pub struct DateInput {
    pub input: TextArea<'static>,
    pub selected_date: Date,
    pub input_mode: DateInputMode,
    pub date_format: &'static str,
    pub error_message: Option<String>,
}

impl Default for DateInput {
    fn default() -> Self {
        let (today, warning) = get_today_with_fallbacks();
        let mut input = TextArea::default();

        input.insert_str(
            today
                .format(format_description!("[year]-[month]-[day]"))
                .unwrap(),
        );

        Self {
            input,
            selected_date: today,
            input_mode: DateInputMode::Text,
            date_format: "[year]-[month]-[day]",
            error_message: warning,
        }
    }
}

impl DateInput {
    pub fn new() -> Self {
        DateInput::default()
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        match self.input_mode {
            DateInputMode::Calendar => self.handle_calendar_input(key),
            DateInputMode::Text => self.handle_text_input(key),
        }
    }

    pub fn toggle_date_input_mode(&mut self) {
        match self.input_mode {
            DateInputMode::Calendar => self.input_mode = DateInputMode::Text,
            DateInputMode::Text => self.input_mode = DateInputMode::Calendar,
        }
    }

    fn handle_text_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c' | 'C') if key.modifiers == KeyModifiers::CONTROL => {
                self.input_mode = DateInputMode::Calendar;
                if let Ok(date) = Date::parse(
                    &self.input.lines()[0],
                    &time::format_description::parse(self.date_format).unwrap(),
                ) {
                    self.selected_date = date;
                }
            }
            _ => {
                self.input.input(key);
            }
        }
    }
    fn handle_calendar_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = DateInputMode::Text;
            }
            KeyCode::Enter => {
                let date_str = self
                    .selected_date
                    .format(&time::format_description::parse(self.date_format).unwrap())
                    .unwrap_or_default();
                self.input = TextArea::default();
                self.input.insert_str(&date_str);
                self.input_mode = DateInputMode::Text;
            }
            KeyCode::Char('h') => {
                self.selected_date = self
                    .selected_date
                    .previous_day()
                    .unwrap_or(self.selected_date);
            }
            KeyCode::Char('l') => {
                self.selected_date = self.selected_date.next_day().unwrap_or(self.selected_date);
            }
            KeyCode::Char('j') => {
                self.selected_date = self
                    .selected_date
                    .checked_add(Duration::days(7))
                    .unwrap_or(self.selected_date);
            }
            KeyCode::Char('k') => {
                self.selected_date = self
                    .selected_date
                    .checked_sub(Duration::days(7))
                    .unwrap_or(self.selected_date);
            }

            KeyCode::Char('p') => {
                let month_num = u8::from(self.selected_date.month());

                if month_num > 1 {
                    if let Ok(month) = Month::try_from(month_num - 1) {
                        let year = self.selected_date.year();
                        let day =
                            u8::min(self.selected_date.day(), days_in_month(year, month_num - 1));
                        if let Ok(date) = Date::from_calendar_date(year, month, day) {
                            self.selected_date = date;
                        }
                    }
                } else if let Ok(date) = Date::from_calendar_date(
                    self.selected_date.year() - 1,
                    Month::December,
                    self.selected_date.day().min(31),
                ) {
                    self.selected_date = date;
                }
            }

            KeyCode::Char('n') => {
                let month_num = u8::from(self.selected_date.month());

                if month_num < 12 {
                    if let Ok(month) = Month::try_from(month_num + 1) {
                        let year = self.selected_date.year();
                        let day =
                            u8::min(self.selected_date.day(), days_in_month(year, month_num + 1));
                        if let Ok(date) = Date::from_calendar_date(year, month, day) {
                            self.selected_date = date;
                        }
                    }
                } else if let Ok(date) = Date::from_calendar_date(
                    self.selected_date.year() + 1,
                    Month::January,
                    self.selected_date.day().min(31),
                ) {
                    self.selected_date = date;
                }
            }

            KeyCode::Char('N') => {
                let year = self.selected_date.year() + 1;
                let month = self.selected_date.month();
                let day = u8::min(
                    self.selected_date.day(),
                    days_in_month(year, u8::from(month)),
                );

                if let Ok(date) = Date::from_calendar_date(year, month, day) {
                    self.selected_date = date
                }
            }
            KeyCode::Char('P') => {
                let year = self.selected_date.year() - 1;
                let month = self.selected_date.month();
                let day = u8::min(
                    self.selected_date.day(),
                    days_in_month(year, u8::from(month)),
                );

                if let Ok(date) = Date::from_calendar_date(year, month, day) {
                    self.selected_date = date
                }
            }

            KeyCode::Char('t') => {
                self.selected_date = OffsetDateTime::now_local()
                    .unwrap_or(OffsetDateTime::now_utc())
                    .date();
            }
            _ => {}
        }
    }

    pub fn render(
        &mut self,
        total_area: Rect,
        input_area: Rect,
        buf: &mut Buffer,
        border_style: Style,
        cursor_style: Style,
    ) {
        let input_block = Block::default()
            .title(Line::from("Due Date: (YYYY-MM-DD)").left_aligned())
            .title(Line::from("Press <Ctrl-c> for calendar view").right_aligned())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style);
        match self.input_mode {
            DateInputMode::Text => {
                self.input.set_block(input_block);
                self.input.set_cursor_style(cursor_style);
                self.input.render(input_area, buf);
            }
            DateInputMode::Calendar => {
                let date_str = self
                    .selected_date
                    .format(&time::format_description::parse(self.date_format).unwrap())
                    .unwrap_or_default();

                Paragraph::new(date_str)
                    .block(input_block)
                    .render(input_area, buf);

                let calendar_area = get_center_rect(20, 9, total_area);
                Clear.render(calendar_area, buf);

                let mut event = CalendarEventStore::default();
                event.add(self.selected_date, Style::default().fg(Color::Yellow));

                let calendar = Monthly::new(self.selected_date, event)
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .style(Style::default().bg(Color::Black)),
                    )
                    .show_month_header(Style::default())
                    .show_weekdays_header(Style::default());

                calendar.render(calendar_area, buf);
            }
        }
    }
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            // February - check for leap year
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}

use chrono::{DateTime, Local};
use crossterm::event::KeyEvent;
use ratatui::style::{Color, Style};
use strum::{Display, EnumIter, IntoEnumIterator};
use tui_textarea::TextArea;

#[derive(Debug)]
pub struct TaskForm {
    pub is_open: bool,
    pub selected: FormField,
    pub form_input: FormInput,
}

#[derive(Debug, EnumIter, Display, Clone, Copy, Eq, PartialEq)]
pub enum FormField {
    Title,
    Description,
    Group,
    DueDate,
}

impl FormField {
    pub fn next(&self) -> Self {
        let all: Vec<_> = FormField::iter().collect();
        let current_position = all.iter().position(|s| s == self).unwrap();
        let next_position = (current_position + 1) % all.len();
        all[next_position]
    }

    pub fn previous(&self) -> Self {
        let all: Vec<_> = FormField::iter().collect();
        let current_position = all.iter().position(|s| s == self).unwrap();
        let previous_position = (current_position + all.len() - 1) % all.len();
        all[previous_position]
    }
}

pub trait FormFieldAccess<T> {
    fn access_field<'a, 'b>(&'a self, form: &'b mut FormInput) -> Option<&'b mut T>;
}

impl FormFieldAccess<TextArea<'static>> for FormField {
    fn access_field<'a, 'b>(
        &'a self,
        form: &'b mut FormInput,
    ) -> Option<&'b mut TextArea<'static>> {
        match self {
            FormField::Title => Some(&mut form.title),
            FormField::Description => Some(&mut form.description),
            _ => None,
        }
    }
}

impl FormFieldAccess<String> for FormField {
    fn access_field<'a, 'b>(&'a self, form: &'b mut FormInput) -> Option<&'b mut String> {
        match self {
            FormField::Group => Some(&mut form.group),
            _ => None,
        }
    }
}

impl FormFieldAccess<DateTime<Local>> for FormField {
    fn access_field<'a, 'b>(&'a self, form: &'b mut FormInput) -> Option<&'b mut DateTime<Local>> {
        match self {
            FormField::DueDate => Some(&mut form.due_date),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct FormInput {
    pub title: TextArea<'static>,
    pub description: TextArea<'static>,
    pub group: String,
    pub due_date: DateTime<Local>,
}

impl Default for FormInput {
    fn default() -> Self {
        Self {
            title: TextArea::default(),
            description: TextArea::default(),
            group: String::new(),
            due_date: Local::now(),
        }
    }
}

impl Default for TaskForm {
    fn default() -> Self {
        Self {
            is_open: false,
            selected: FormField::Title,
            form_input: FormInput::default(),
        }
    }
}

impl TaskForm {
    pub fn toggle_task_form(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn select_next(&mut self) {
        self.selected = self.selected.next();
    }

    pub fn select_previous(&mut self) {
        self.selected = self.selected.previous();
    }

    pub fn access_current_field<T>(&mut self) -> Option<&mut T>
    where
        FormField: FormFieldAccess<T>,
    {
        self.selected.access_field(&mut self.form_input)
    }

    pub fn get_field<T>(&mut self, field: FormField) -> Option<&mut T>
    where
        FormField: FormFieldAccess<T>,
    {
        field.access_field(&mut self.form_input)
    }

    pub fn input(&mut self, key: KeyEvent) {
        match self.selected {
            FormField::Title | FormField::Description => {
                if let Some(textarea) = self.access_current_field::<TextArea<'static>>() {
                    textarea.input(key);
                }
            }

            _ => {}
        }
    }

    pub fn get_input_border_style(&self, field: FormField) -> Style {
        if self.selected == field {
            return Style::default().fg(Color::LightYellow);
        }
        Style::default().fg(Color::LightCyan)
    }

    pub fn get_cursor_style(&self, field: FormField) -> Style {
        if self.selected == field {
            return Style::default().bg(Color::White);
        }
        Style::default().bg(Color::Black)
    }
}

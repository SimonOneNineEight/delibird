use std::collections::HashMap;

use crossterm::event::KeyEvent;
use ratatui::style::{Color, Style};
use strum::{Display, EnumIter, IntoEnumIterator};
use time::Date;
use tui_textarea::TextArea;

use crate::core::{error::AppError, task::Task, validation::TaskValidator};

use super::date_input::DateInput;

#[derive(Debug)]
pub struct TaskForm {
    pub is_open: bool,
    pub selected: FormField,
    pub form_input: FormInput,
    pub field_errors: HashMap<FormField, String>,
}

#[derive(Debug, EnumIter, Display, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FormField {
    Title,
    DueDate,
    Description,
    // Group,
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

// impl FormFieldAccess<String> for FormField {
//     fn access_field<'a, 'b>(&'a self, form: &'b mut FormInput) -> Option<&'b mut String> {
//         match self {
//             FormField::Group => Some(&mut form.group),
//             _ => None,
//         }
//     }
// }

impl FormFieldAccess<DateInput> for FormField {
    fn access_field<'a, 'b>(&'a self, form: &'b mut FormInput) -> Option<&'b mut DateInput> {
        match self {
            FormField::DueDate => Some(&mut form.due_date),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormInput {
    pub title: TextArea<'static>,
    pub description: TextArea<'static>,
    pub group: String,
    pub due_date: DateInput,
}

#[derive(Debug, Clone)]
pub struct FormInputData {
    pub title: String,
    pub description: Vec<String>,
    pub due_date: Date,
}

impl Default for FormInput {
    fn default() -> Self {
        Self {
            title: TextArea::default(),
            description: TextArea::default(),
            group: String::new(),
            due_date: DateInput::new(),
        }
    }
}

impl Default for TaskForm {
    fn default() -> Self {
        Self {
            is_open: false,
            selected: FormField::Title,
            form_input: FormInput::default(),
            field_errors: HashMap::new(),
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

            FormField::DueDate => {
                if let Some(date_input) = self.access_current_field::<DateInput>() {
                    date_input.handle_input(key);
                }
            }
        }
    }

    pub fn reset_form_input(&mut self) {
        self.form_input = FormInput::default();
        self.selected = FormField::Title;
    }

    pub fn validate_current_field(&mut self) -> Result<(), (FormField, String)> {
        match self.selected {
            FormField::Title => {
                let title = &self.form_input.title.lines()[0];
                match TaskValidator::validate_title(title) {
                    Ok(_) => Ok(()),
                    Err(app_error) => Err((FormField::Title, app_error.user_message())),
                }
            }
            FormField::DueDate => {
                let selected_date = self.form_input.due_date.selected_date;

                match TaskValidator::validate_due_date(selected_date) {
                    Ok(_) => Ok(()),
                    Err(app_error) => Err((FormField::DueDate, app_error.user_message())),
                }
            }
            FormField::Description => {
                let description: Vec<String> = self
                    .form_input
                    .description
                    .lines()
                    .iter()
                    .map(|s| s.to_string())
                    .collect();

                match TaskValidator::validate_description(&description) {
                    Ok(_) => Ok(()),
                    Err(app_error) => Err((FormField::Description, app_error.user_message())),
                }
            }
        }
    }

    pub fn validate_all_field(&mut self) -> Result<(), HashMap<FormField, String>> {
        let title = &self.form_input.title.lines()[0];
        let description = self.form_input.description.lines();
        let due_date = self.form_input.due_date.selected_date;

        TaskValidator::validate_all_task_field(title, description, due_date)
    }

    pub fn clear_field_errors(&mut self) {
        self.field_errors = HashMap::new();
    }

    pub fn get_input_border_style(&self, field: FormField) -> Style {
        if self.field_errors.contains_key(&field) {
            return Style::default().fg(Color::Red);
        }
        if self.selected == field {
            return Style::default().fg(Color::LightYellow);
        }
        Style::default().fg(Color::White)
    }

    pub fn get_cursor_style(&self, field: FormField) -> Style {
        if self.selected == field {
            return Style::default().bg(Color::White);
        }
        Style::default().bg(Color::Black)
    }

    pub fn to_task_data(&mut self) -> FormInputData {
        FormInputData {
            title: self.form_input.title.lines()[0].clone(),
            description: self
                .form_input
                .description
                .lines()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            due_date: self.form_input.due_date.selected_date,
        }
    }
}

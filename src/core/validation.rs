use std::collections::HashMap;

use time::{Date, Duration, format_description, macros::format_description};
use uuid::Uuid;

use crate::{ui::forms::task_form::FormField, utils::date::get_today_with_fallbacks};

use super::error::AppError;

pub type ValidationResult<T> = Result<T, AppError>;

pub struct TaskValidator;

impl TaskValidator {
    pub fn validate_title(title: &str) -> ValidationResult<String> {
        let trimmed = title.trim();

        if trimmed.is_empty() {
            return Err(AppError::EmptyTaskTitle);
        }

        if trimmed.len() > 200 {
            return Err(AppError::InvalidTitle {
                reason: "Title cannot exceed 200 characters".to_string(),
            });
        }

        if trimmed.contains('\n') || trimmed.contains('\t') {
            return Err(AppError::InvalidTitle {
                reason: "Title cannot contain newlines or tabs".to_string(),
            });
        }

        Ok(trimmed.to_string())
    }

    pub fn validate_description(description: &[String]) -> ValidationResult<Vec<String>> {
        let mut validated_lines = Vec::new();

        for (i, line) in description.iter().enumerate() {
            if line.len() > 500 {
                return Err(AppError::InvalidDescription {
                    reason: format!("Line {} exceeds 500 characters", i + 1),
                });
            }
            validated_lines.push(line.clone());
        }

        if validated_lines.len() > 20 {
            return Err(AppError::InvalidDescription {
                reason: "Description cannot exceed 20 lines".to_string(),
            });
        }

        Ok(validated_lines)
    }

    pub fn validate_due_date(date: Date) -> ValidationResult<Date> {
        let today = get_today_with_fallbacks().0;

        if date < today {
            return Err(AppError::InvalidDate {
                input: date.to_string(),
                expected_format: "Date cannot be in the past".to_string(),
            });
        }

        if date > today.saturating_add(Duration::days(365)) {
            return Err(AppError::InvalidDate {
                input: date.to_string(),
                expected_format: "Date cannot be more than 1 year in the future".to_string(),
            });
        }

        Ok(date)
    }

    pub fn validate_task_data(
        title: &str,
        description: &[String],
        due_date: Date,
    ) -> ValidationResult<(String, Vec<String>, Date)> {
        let validated_title = Self::validate_title(title)?;
        let validated_description = Self::validate_description(description)?;
        let validated_date = Self::validate_due_date(due_date)?;

        Ok((validated_title, validated_description, validated_date))
    }

    pub fn validate_task_id(task_id: Uuid, existing_ids: &[Uuid]) -> ValidationResult<Uuid> {
        if existing_ids.contains(&task_id) {
            Ok(task_id)
        } else {
            Err(AppError::TaskNotFound { id: task_id })
        }
    }

    pub fn validate_all_task_field(
        title: &str,
        description: &[String],
        due_date: Date,
    ) -> Result<(), HashMap<FormField, String>> {
        let mut field_errors = HashMap::<FormField, String>::new();

        if let Err(app_error) = TaskValidator::validate_title(title) {
            field_errors.insert(FormField::Title, app_error.user_message());
        };

        if let Err(app_error) = TaskValidator::validate_description(description) {
            field_errors.insert(FormField::Description, app_error.user_message());
        }

        if let Err(app_error) = TaskValidator::validate_due_date(due_date) {
            field_errors.insert(FormField::DueDate, app_error.user_message());
        }

        if field_errors.is_empty() {
            Ok(())
        } else {
            Err(field_errors)
        }
    }
}

pub struct DateValidator;

impl DateValidator {
    pub fn parse_date_string(input: &str, format: &str) -> ValidationResult<Date> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(AppError::InvalidDate {
                input: input.to_string(),
                expected_format: format.to_string(),
            });
        }

        let format_desc = format_description::parse(format).map_err(|_| AppError::InvalidDate {
            input: input.to_string(),
            expected_format: format.to_string(),
        })?;

        let parsed_date =
            Date::parse(trimmed, &format_desc).map_err(|_| AppError::InvalidDate {
                input: input.to_string(),
                expected_format: format.to_string(),
            })?;

        TaskValidator::validate_due_date(parsed_date)
    }

    pub fn validate_date_input(input: &str) -> ValidationResult<Date> {
        Self::parse_date_string(input, "[year]-[month]-[day]")
    }
}

pub struct FormValidator;

impl FormValidator {
    pub fn validate_task_form(
        title: &str,
        description: &[String],
        due_date_input: &str,
    ) -> ValidationResult<(String, Vec<String>, Date)> {
        let validated_title = TaskValidator::validate_title(title)?;
        let validated_description = TaskValidator::validate_description(description)?;
        let validated_date = DateValidator::validate_date_input(due_date_input)?;

        Ok((validated_title, validated_description, validated_date))
    }
}

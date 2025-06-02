use std::process::{Command, Output};

use time::{Date, OffsetDateTime, macros::format_description};

pub fn get_today_with_fallbacks() -> (Date, Option<String>) {
    if let Ok(local_date) = OffsetDateTime::now_local() {
        return (local_date.date(), None);
    }

    if let Ok(system_date) = get_system_date() {
        return (
            system_date,
            Some("Using system date (timezone detection unavailable)".to_string()),
        );
    }

    let utc_time = OffsetDateTime::now_utc();
    (
        utc_time.date(),
        Some("Using UTC date - please verify this is correct for your timezone".to_string()),
    )
}

pub fn get_system_date() -> Result<Date, String> {
    #[cfg(not(any(unix, windows)))]
    {
        Err("System date not supported on this platform".to_string())
    }
    let output: Output;
    #[cfg(unix)]
    {
        output = Command::new("date")
            .arg("+%Y-%m-%d")
            .output()
            .map_err(|err| format!("System date command fail: {}", err))?;
    }

    #[cfg(windows)]
    {
        output = Command::new("powershell")
            .args(&["-Command", "Get-Date -Format yyyy-MM-dd"])
            .output()
            .map_err(|err| format!("PowerShell date command error: {}", err))?;
    }

    if output.status.success() {
        let date_str = String::from_utf8(output.stdout)
            .map_err(|err| format!("Invalid date output: {}", err))?
            .trim()
            .to_string();
        let description = format_description!("[year]-[month]-[day]");

        Date::parse(&date_str, &description)
            .map_err(|err| format!("Failed to parse system date: {}", err))
    } else {
        Err("Date command returned error".to_string())
    }
}

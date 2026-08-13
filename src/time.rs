use std::time::SystemTime;

use chrono::{DateTime, Duration, Utc};

pub fn now(back: i64) -> DateTime<Utc> {
    let now: DateTime<Utc> = SystemTime::now().into();
    now - Duration::hours(back)
}

pub fn parse(val: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc2822(val)
        .or_else(|_| DateTime::parse_from_rfc3339(val))
        .map_err(|e| e.to_string())
        .map(|d| d.with_timezone(&Utc))
}

#[derive(Debug)]
pub struct DateTimeUtc(pub DateTime<Utc>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_date() {
        assert!(parse("not-a-date").is_err());
    }

    #[test]
    fn supported_dates() {
        let dates = [
            "Mon, 02 Jan 2006 15:04:05 GMT",
            "Mon, 02 Jan 2006 15:04:05 -0700",
            "2006-01-02T15:04:05-07:00",
            "Mon, 2 Jan 2006 15:04:05 MST",
            "Mon, 2 Jan 2006 15:04:05 -0700",
        ];

        for value in dates {
            assert!(parse(value).is_ok());
        }
    }
}

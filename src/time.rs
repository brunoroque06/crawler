use chrono::{DateTime, Duration, Utc};
use std::{fmt::Display, time::SystemTime};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTimeUtc(DateTime<Utc>);

impl Display for DateTimeUtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M"))
    }
}

impl DateTimeUtc {
    pub fn hours_ago(back: i64) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        Self(now - Duration::hours(back))
    }

    pub fn parse(val: &str) -> Result<Self, String> {
        DateTime::parse_from_rfc2822(val)
            .or_else(|_| DateTime::parse_from_rfc3339(val))
            .map_err(|e| e.to_string())
            .map(|d| Self(d.with_timezone(&Utc)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_date() {
        assert!(DateTimeUtc::parse("not-a-date").is_err());
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
            assert!(DateTimeUtc::parse(value).is_ok());
        }
    }
}

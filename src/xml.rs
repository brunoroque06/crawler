use serde::{Deserialize, Deserializer, de};

use crate::time::{DateTimeUtc, parse};

pub fn from_str<'de, T>(s: &'de str) -> Result<T, String>
where
    T: Deserialize<'de>,
{
    quick_xml::de::from_str(s).map_err(|e| e.to_string())
}

impl<'de> Deserialize<'de> for DateTimeUtc {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = String::deserialize(des)?;
        parse(&val)
            .map_err(|_| de::Error::custom(format!("invalid date: {val}")))
            .map(|d| Self(d))
    }
}

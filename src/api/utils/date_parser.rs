use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer};

pub fn deserialize_optional_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;

    match opt {
        Some(s) => {
            // Try RFC3339 first
            if let Ok(dt) = s.parse::<DateTime<Utc>>() {
                return Ok(Some(dt));
            }

            // Fallback to YYYY-MM-DD
            let date = NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .map_err(serde::de::Error::custom)?;

            Ok(Some(Utc.from_utc_datetime(
                &date.and_hms_opt(0, 0, 0).unwrap(),
            )))
        }
        None => Ok(None),
    }
}
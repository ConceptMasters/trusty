use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timestamps {
    #[serde(serialize_with = "serialize_datetime")]
    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_optional_datetime")]
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Timestamps {
    pub fn new() -> Self {
        Timestamps {
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update(&mut self) {
        self.updated_at = Some(Utc::now());
    }
}

pub fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = dt.to_rfc3339();
    serializer.serialize_str(&formatted)
}

pub fn serialize_optional_datetime<S>(
    odt: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match odt {
        Some(dt) => {
            let formatted = dt.to_rfc3339();
            serializer.serialize_str(&formatted)
        }
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime_str = String::deserialize(deserializer)?;
    match DateTime::parse_from_rfc3339(&datetime_str) {
        Ok(parsed_datetime) => Ok(parsed_datetime.with_timezone(&Utc)),
        Err(err) => Err(de::Error::custom(format!(
            "Failed to parse datetime: {}",
            err
        ))),
    }
}

pub fn deserialize_optional_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime_str: Option<String> = Option::deserialize(deserializer)?;
    match datetime_str {
        Some(d) => match DateTime::parse_from_rfc3339(&d) {
            Ok(datetime) => Ok(Some(datetime.with_timezone(&Utc))),
            Err(err) => Err(serde::de::Error::custom(format!(
                "Failed to parse datetime: {}",
                err
            ))),
        },
        None => Ok(None),
    }
}

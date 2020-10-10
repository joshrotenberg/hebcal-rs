use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{de::Error, Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Location {
    pub title: String,
    pub city: String,
    pub tzid: String,
    pub latitude: f64,
    pub longitude: f64,
    pub cc: String,
    pub country: String,
    pub admin1: String,
    pub geo: String,
    pub geonameid: Option<u32>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Item {
    pub title: String,
    #[serde(deserialize_with = "deserialize_item_date")]
    pub date: chrono::NaiveDateTime,
    pub category: Option<String>,
    pub subcat: Option<String>,
    pub hebrew: String,
    pub link: Option<String>,
    pub memo: Option<String>,
    pub title_orig: Option<String>,
    pub yomtov: Option<bool>,
    // XXX: should be able to make these actual values, revisit
    pub leyning: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Shabbat {
    pub title: Option<String>,
    pub date: chrono::DateTime<chrono::Utc>,
    pub location: Location,
    pub items: Vec<Item>,
}
fn deserialize_item_date<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error> {
    let date: String = Deserialize::deserialize(deserializer)?;

    match NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S%z") {
        Ok(r) => Ok(r),
        Err(_) => {
            let empty_time = NaiveTime::from_hms(0, 0, 0);
            let new_date =
                NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(D::Error::custom)?;
            Ok(NaiveDateTime::new(new_date, empty_time))
        }
    }
}

use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Geo {
    Geoname,
    Zip,
    City,
    Pos,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Leyning {
    On,
    Off,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Transliteration {
    #[serde(rename = "on")]
    Ashkenazic,
    #[serde(rename = "off")]
    Sephardic,
}

#[derive(Debug, Serialize, Default)]
pub struct ShabbatOptions {
    #[serde(rename = "m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub havdalah: Option<u16>,
    #[serde(rename = "b")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_before_sunset: Option<u16>,
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transliteration: Option<Transliteration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leyning: Option<Leyning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,
    #[serde(rename = "geonameid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoname_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tzid: Option<String>,

    #[serde(rename = "gy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gregorian_year: Option<i32>,
    #[serde(rename = "gm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gregorian_month: Option<u32>,
    #[serde(rename = "gd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gregorian_day: Option<u32>,
}

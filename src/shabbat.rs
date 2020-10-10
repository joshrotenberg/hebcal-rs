use crate::HebCal;
use anyhow::Result;

mod options;
mod response;

pub use self::options::{Geo, Leyning, ShabbatOptions, Transliteration};
pub use self::response::Shabbat;

#[derive(Debug)]
pub struct ShabbatHandler<'hc> {
    client: &'hc HebCal,
    options: ShabbatOptions,
}

impl<'hc> ShabbatHandler<'hc> {
    pub fn new(hebcal: &'hc HebCal) -> Self {
        Self {
            client: hebcal,
            options: ShabbatOptions::default(),
        }
    }

    pub fn havdalah(mut self, havdalah: impl Into<u16>) -> Self {
        self.options.havdalah = Some(havdalah.into());
        self
    }

    pub fn minutes_before_sunset(mut self, minutes_before_sunset: impl Into<u16>) -> Self {
        self.options.minutes_before_sunset = Some(minutes_before_sunset.into());
        self
    }

    pub fn transliteration(mut self, transliteration: impl Into<Transliteration>) -> Self {
        self.options.transliteration = Some(transliteration.into());
        self
    }

    pub fn leyning(mut self, leyning: impl Into<Leyning>) -> Self {
        self.options.leyning = Some(leyning.into());
        self
    }

    pub fn geo(mut self, geo: impl Into<Geo>) -> Self {
        self.options.geo = Some(geo.into());
        self
    }

    pub fn geoname_id(mut self, geoname_id: impl Into<u32>) -> Self {
        self.options.geoname_id = Some(geoname_id.into());
        self
    }

    pub fn zip(mut self, zip: impl Into<String>) -> Self {
        self.options.zip = Some(zip.into());
        self
    }

    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.options.city = Some(city.into());
        self
    }

    pub fn latitude(mut self, latitude: impl Into<f32>) -> Self {
        self.options.latitude = Some(latitude.into());
        self
    }

    pub fn longitude(mut self, longitude: impl Into<f32>) -> Self {
        self.options.longitude = Some(longitude.into());
        self
    }

    pub fn tzid(mut self, tzid: impl Into<String>) -> Self {
        self.options.tzid = Some(tzid.into());
        self
    }

    pub fn gregorian_year(mut self, gregorian_year: impl Into<i32>) -> Self {
        self.options.gregorian_year = Some(gregorian_year.into());
        self
    }

    pub fn gregorian_month(mut self, gregorian_month: impl Into<u32>) -> Self {
        self.options.gregorian_month = Some(gregorian_month.into());
        self
    }

    pub fn gregorian_day(mut self, gregorian_day: impl Into<u32>) -> Self {
        self.options.gregorian_day = Some(gregorian_day.into());
        self
    }

    pub async fn send(&self) -> Result<Shabbat> {
        Ok(self
            .client
            .get("/shabbat", Some(&self.options))
            .await?
            .json()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shabbat_handler() -> Result<()> {
        let client = HebCal::default();
        let shabbat = ShabbatHandler::new(&client)
            .zip("94706")
            .gregorian_month(6u32)
            .gregorian_year(2001)
            .gregorian_day(13u32)
            .send()
            .await?;
        dbg!(shabbat);
        Ok(())
    }
}

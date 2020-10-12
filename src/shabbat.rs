use crate::{HebCal, Result};

mod options;
mod response;

pub use self::options::{Geo, Leyning, ShabbatOptions, Transliteration};
pub use self::response::Shabbat;

/// Shabbat API handler
#[derive(Debug)]
pub struct ShabbatHandler<'hc> {
    client: &'hc HebCal,
    options: ShabbatOptions,
}

/// Shabbat times API handler
///
/// See the API [documentation](https://www.hebcal.com/home/197/shabbat-times-rest-api) for more details on the options provided
impl<'hc> ShabbatHandler<'hc> {
    pub(crate) fn new(hebcal: &'hc HebCal) -> Self {
        Self {
            client: hebcal,
            options: ShabbatOptions::default(),
        }
    }

    /// Set the havdalah time in minutes
    pub fn havdalah(mut self, havdalah: impl Into<u16>) -> Self {
        self.options.havdalah = Some(havdalah.into());
        self
    }

    /// Candle lighting time minutes before sunset
    pub fn minutes_before_sunset(mut self, minutes_before_sunset: impl Into<u16>) -> Self {
        self.options.minutes_before_sunset = Some(minutes_before_sunset.into());
        self
    }

    /// Use Sephardic or Ashkenazic transliterations
    pub fn transliteration(mut self, transliteration: impl Into<Transliteration>) -> Self {
        self.options.transliteration = Some(transliteration.into());
        self
    }

    /// Inlcude Torah and Haftarah for regular Shabbat
    pub fn leyning(mut self, leyning: impl Into<Leyning>) -> Self {
        self.options.leyning = Some(leyning.into());
        self
    }

    /// Specify the geographical location methodology. This is also set automatically when using any of
    /// the geographic specific options (geoname, zip, city or position) so shouldn't need to be set directly
    pub fn geo(mut self, geo: impl Into<Geo>) -> Self {
        self.options.geo = Some(geo.into());
        self
    }

    /// Specify the geoname id for the desired location. See [GeoNames](http://www.geonames.org) for more information
    pub fn geoname_id(mut self, geoname_id: impl Into<u32>) -> Self {
        self.options.geoname_id = Some(geoname_id.into());
        self.options.geo = Some(Geo::Geoname);
        self
    }

    // Specify the 5 digit US zip code for the desired location
    pub fn zip(mut self, zip: impl Into<String>) -> Self {
        self.options.zip = Some(zip.into());
        self.options.geo = Some(Geo::Zip);
        self
    }

    /// Specify a city location using the legacy city identifier, such as GB-London
    /// See https://github.com/hebcal/dotcom/blob/master/hebcal.com/dist/cities2.txt
    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.options.city = Some(city.into());
        self.options.geo = Some(Geo::City);
        self
    }

    /// Specify a latitude in decimal format, such as 31.76904 or -23.5475
    pub fn latitude(mut self, latitude: impl Into<f32>) -> Self {
        self.options.latitude = Some(latitude.into());
        self.options.geo = Some(Geo::Pos);
        self
    }

    /// Specify a longitude in decimal format, such as 35.21633 or -46.63611
    pub fn longitude(mut self, longitude: impl Into<f32>) -> Self {
        self.options.longitude = Some(longitude.into());
        self.options.geo = Some(Geo::Pos);
        self
    }

    /// Specify a timezone identifier string, such as America/Los_Angeles
    /// See https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
    pub fn tzid(mut self, tzid: impl Into<String>) -> Self {
        self.options.tzid = Some(tzid.into());
        self.options.geo = Some(Geo::Pos);
        self
    }

    /// Specify a specific year
    pub fn gregorian_year(mut self, gregorian_year: impl Into<i32>) -> Self {
        self.options.gregorian_year = Some(gregorian_year.into());
        self
    }

    /// Specify a specific month
    pub fn gregorian_month(mut self, gregorian_month: impl Into<u32>) -> Self {
        self.options.gregorian_month = Some(gregorian_month.into());
        self
    }

    /// Specify a specific day
    pub fn gregorian_day(mut self, gregorian_day: impl Into<u32>) -> Self {
        self.options.gregorian_day = Some(gregorian_day.into());
        self
    }

    /// Send the request
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

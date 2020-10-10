///! # HebCal: hebcal.com REST API client
///! HebCal is an API client for several of the RESTful APIs offered at https://www.hebcal.com/home/developer-apis
///! 
/// 
/// 
use self::shabbat::ShabbatHandler;
use anyhow::Result;
use reqwest::{Client, Response, Url};
use serde::{Deserialize, Serialize};
use std::io::Error;

const HEBCAL_BASE_URL: &str = "https://www.hebcal.com";

pub mod shabbat;

/// The main client struct
#[derive(Debug, Clone)]
pub struct HebCal {
    client: Client,
    base_url: Url,
}

impl Default for HebCal {
    fn default() -> Self {
        Self {
            base_url: Url::parse(HEBCAL_BASE_URL).unwrap(),
            client: reqwest::ClientBuilder::new().build().unwrap(),
        }
    }
}

impl HebCal {
    pub fn shabbat(&self) -> ShabbatHandler {
        ShabbatHandler::new(self)
    }

    pub async fn get<P>(&self, path: impl AsRef<str>, params: Option<&P>) -> Result<Response>
    where
        P: Serialize + ?Sized,
    {
        let url = self.base_url.join(path.as_ref())?;
        let mut req = self.client.get(url).query(&[("cfg", "json")]);
        if let Some(p) = params {
            req = req.query(p);
        }
        dbg!(&req);
        let res = req.send().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

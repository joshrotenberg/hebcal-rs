//! ## HebCal: hebcal.com REST API client
//! HebCal is an API client for several of the RESTful APIs offered at https://www.hebcal.com/home/developer-apis
//!
//! ### Shabbat API
//!
//! The Shabbat times API returns location specific Shabbat times and details.
//!
//! ```no_run
//! use hebcal::{
//!     shabbat::{Leyning, Transliteration},
//!     HebCal, Result,
//! };
//!
//! async fn f() -> Result<(), Box<dyn std::error::Error>> {
//!     let hebcal = HebCal::default();
//!     let shabbat = hebcal
//!      .shabbat()
//!      .transliteration(Transliteration::Ashkenazic)
//!      .zip("94706")
//!      .leyning(Leyning::Off)
//!      .send()
//!      .await?;
//!     
//!     // do stuff with `shabbat`
//!
//!     Ok(())
//! }
//!
//! ```
//!
use self::{error::HebCalError, shabbat::ShabbatHandler};
use reqwest::{Client, Response, Url};
use serde::Serialize;

mod error;
pub mod shabbat;

const HEBCAL_BASE_URL: &str = "https://www.hebcal.com";
const HEBCAL_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

/// The client
#[derive(Debug, Clone)]
pub struct HebCal {
    client: Client,
    base_url: Url,
}

impl Default for HebCal {
    fn default() -> Self {
        Self {
            base_url: Url::parse(HEBCAL_BASE_URL).unwrap(),
            client: reqwest::ClientBuilder::new()
                .user_agent(HEBCAL_USER_AGENT)
                .build()
                .unwrap(),
        }
    }
}

impl HebCal {
    /// Returns a handler for the Shabbat times API
    pub fn shabbat(&self) -> ShabbatHandler {
        ShabbatHandler::new(self)
    }

    pub(crate) async fn get<P>(&self, path: impl AsRef<str>, params: Option<&P>) -> Result<Response>
    where
        P: Serialize + ?Sized,
    {
        let url = self.base_url.join(path.as_ref())?;
        let mut req = self.client.get(url).query(&[("cfg", "json")]);
        if let Some(p) = params {
            req = req.query(p);
        }
        
        let res = req.send().await?;
        if res.status().is_success() {
            Ok(res)
        } else {
            let e: HebCalError = res.json::<HebCalError>().await?;
            Err(error::Error::HebCal { error: e })
        }
    }
}


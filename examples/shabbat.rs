use anyhow::Result;
use hebcal::{
    shabbat::Transliteration,
    HebCal,
};

#[tokio::main]
async fn main() -> Result<()> {
    let hebcal = HebCal::default();
    let shabbat = hebcal
        .shabbat()
        .transliteration(Transliteration::Ashkenazic)
        .zip("94706")
        .havdalah(20u16)
        .send()
        .await?;
    println!("{:#?}", shabbat);
    Ok(())
}

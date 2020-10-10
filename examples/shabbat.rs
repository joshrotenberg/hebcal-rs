use anyhow::Result;
use hebcal::HebCal;

#[tokio::main]
async fn main() -> Result<()> {
    let hebcal = HebCal::default();
    let shabbat = hebcal.shabbat().send().await?;
    println!("{:#?}", shabbat);
    Ok(())
}

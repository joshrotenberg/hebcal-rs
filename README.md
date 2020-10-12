# hebcal-rs: hebcal.com client for Shabbat times

[![crates.io](https://img.shields.io/crates/d/hebcal.svg)](https://crates.io/crates/hebcal)
[![Rust](https://github.com/joshrotenberg/hebcal-rs/workflows/Rust/badge.svg)](https://github.com/joshrotenberg/hebcal-rs/actions?query=workflow%3ARust)

`hebcal-rs` is a client library in Rust for requesting Shabbat times from the API at `http://www.hebcal.com`. 

## Cargo
```toml
hebcal = "0.1"
```

## Example
```rust
use anyhow::Result;
use hebcal::{
    shabbat::Transliteration,
    HebCal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
```
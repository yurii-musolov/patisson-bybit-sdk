//! Run with
//!
//! ```not_rust
//! cargo example rest
//! ```

use bybit::v5::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = String::from("https://api.bybit.com");
    let client = Client::new(url);

    let response = client.get_tickers()?;
    println!("{response}");

    Ok(())
}

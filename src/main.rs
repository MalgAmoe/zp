use clap::{App, Arg};
use dotenv::dotenv;
use reqwest::Error;

mod price;

#[macro_use]
extern crate dotenv_codegen;

fn get_matches(matches: clap::ArgMatches) -> (String, String) {
    let crypto = match matches.value_of("name") {
        Some(name) => name.to_uppercase(),
        None => "BTC".to_string(),
    };
    let currency = match matches.value_of("currency") {
        Some(currency) => currency.to_uppercase(),
        None => "USD".to_string(),
    };
    (crypto, currency)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let matches = App::new("Crypto Prices")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("name").takes_value(true))
        .arg(Arg::with_name("currency").short("c").takes_value(true))
        .get_matches();

    let (crypto, currency) = get_matches(matches);

    price::get_coin_price(&crypto, &currency).await?;

    Ok(())
}

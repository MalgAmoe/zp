use clap::{App, Arg};
use reqwest::Client;
use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use dotenv::dotenv;

#[macro_use]
extern crate dotenv_codegen;

#[derive(Deserialize, Debug)]
struct Price {
    rate: f32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let matches = App::new("Crypto Prices")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("name")
                .takes_value(true),
        )
        .arg(            
            Arg::with_name("currency")
            .short("c")
            .takes_value(true)
        )
        .get_matches();

    let crypto = match matches.value_of("name") {
        Some(name) => name.to_uppercase(),
        None => "BTC".to_string(),
    };
    let currency = match matches.value_of("currency") {
        Some(currency) => currency.to_uppercase(),
        None => "USD".to_string()
    };

    let mut body = HashMap::new();
    body.insert("currency", &currency);
    body.insert("code", &crypto);

    let client = Client::new();
    let res = client
        .post("https://api.livecoinwatch.com/coins/single")
        .header("x-api-key", dotenv!("API_KEY"))
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    let s = res.json().await;
    match s {
        Ok(Price { rate }) => println!("{}: {:?} {}", crypto, rate, currency),
        _ => println!("Try an other coin"),
    }

    Ok(())
}

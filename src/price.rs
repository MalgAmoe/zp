use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Price {
    rate: f32,
}

pub async fn get_coin_price(crypto: &String, currency: &String) -> Result<(), reqwest::Error> {
  let mut body = HashMap::new();
  body.insert("currency", currency);
  body.insert("code", crypto);

  let client = Client::new();
  let res = client
      .post("https://api.livecoinwatch.com/coins/single")
      .header("x-api-key", dotenv!("API_KEY"))
      .header("content-type", "application/json")
      .json(&body)
      .send()
      .await?;

  let json = res.json().await;

  match json {
      Ok(Price { rate }) => println!("{}: {:?} {}", crypto, rate, currency),
      _ => println!("Try an other coin"),
  }
  Ok(())
}
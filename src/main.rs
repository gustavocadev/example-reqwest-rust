// ignore snake case
#![allow(non_snake_case)]
// use packages
use serde::{Deserialize, Serialize};
// use serde_json::Value;
use std::error::Error;

// I am using the serde_json crate to parse the json file
#[derive(Deserialize, Serialize, Debug)]
struct Body {
  body: String,
  id: i32,
  title: String,
  userId: i32,
}

// I am using the serde_json crate to parse the json file
async fn get_data() -> Result<Body, Box<dyn Error>> {
  let url = "https://jsonplaceholder.typicode.com/posts/1";
  let resp = reqwest::get(url).await.unwrap();
  let data: Body = resp.json().await.unwrap();
  Ok(data)
}

// I am using the tokio crate to run the async function
#[tokio::main]
async fn main() {
  let users = get_data().await.unwrap();
  println!("{}", users.body);
}

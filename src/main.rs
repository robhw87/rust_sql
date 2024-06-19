use std::error::Error;
mod db;
mod middleware;
mod models;
mod utils;
use utils::read;

#[tokio::main]
async fn main () -> Result<(), Box<dyn Error>> {
  let pool = &db::POOL;
  let q = "SELECT id, first_name, last_name, birthdate, salary FROM users_list";
  let data = read(&pool, q).await?;
  let serialized = serde_json::to_string(&data)?;
  println!("Serialized: {}", serialized);

  Ok(())
}
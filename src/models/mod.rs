use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String,
  }

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  pub data_id: i32,
  pub first_name: String,
  pub last_name: String,
  pub birthdate: NaiveDate,
  pub salary: i32,
}

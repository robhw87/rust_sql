use sqlx::Row;
use futures::TryStreamExt;
use std::error::Error;
use crate::models::Data;

/*
pub async fn read(conn: &sqlx::PgPool, q:&str) -> Result<Vec<Data>, Box<dyn Error>> {
    let query = sqlx::query(q);
  
      let mut rows = query.fetch(conn);
      let mut datas = vec![];
  
    while let Some(row) = rows.try_next().await? {
      datas.push(Data {
        data_id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        birthdate: row.get("birthdate"),
        salary: row.get("salary"),
      });
    }
  Ok(datas)
  }
  */

  impl 
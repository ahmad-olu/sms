// Copyright (C) 2026 Ahmad Olukotun
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

use anyhow::Result;
use once_cell::sync::OnceCell;
use surrealdb::Surreal;
// use surrealdb::engine::local::Db;
// use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub mod error;
pub mod helpers;
pub mod pagination;
pub mod queries;

static DB: OnceCell<Surreal<Client>> = OnceCell::new();

pub async fn init_db() -> Result<()> {
    let db = Surreal::new::<Ws>("localhost:8011").await?;

    db.signin(Root {
        username: "root".to_string(),
        password: "secret".to_string(),
    })
    .await?;
    db.use_ns("app").use_db("main").await?;

    // db.query(
    //     "
    //     ",
    // )
    // .await?;

    DB.set(db).expect("Database already initialized");

    Ok(())
}

pub fn get_db() -> &'static Surreal<Client> {
    DB.get().expect("Database not initialized")
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
    #[tokio::test]
    async fn test_users_table_exists() {
        //cargo test -- --nocapture
        //cargo test test_users_table_exists -- --nocapture

        // Make sure DB is initialized
        init_db().await.expect("Failed to initialize DB");

        let db = get_db();

        // Query table info
        let table_name = "users";

        let result = db.query(format!("INFO FOR TABLE {}", table_name)).await;

        match result {
            Ok(response) => {
                dbg!("Table info: {:?}", response);
                // You can assert that the response is not empty
                // assert!(!response.is_empty(), "Table '{}' should exist", table_name);
            }
            Err(err) => {
                panic!("Failed to get table info: {}", err);
            }
        }
    }
}

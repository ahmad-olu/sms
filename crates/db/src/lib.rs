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

static DB: OnceCell<Surreal<Client>> = OnceCell::new();

pub async fn init_db() -> Result<()> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root".to_string(),
        password: "root".to_string(),
    })
    .await?;
    db.use_ns("app").use_db("main").await?;

    db.query("
        DEFINE TABLE schools SCHEMAFULL;
        DEFINE FIELD school_name ON schools TYPE string ASSERT $value != NONE;
        DEFINE FIELD school_address ON schools TYPE string;
        DEFINE FIELD school_phone ON schools TYPE string;
        DEFINE FIELD school_email ON schools TYPE string;
        DEFINE FIELD school_logo_url ON schools TYPE string;
        DEFINE FIELD school_motto ON schools TYPE string;
        DEFINE FIELD total_students ON schools TYPE int DEFAULT 0;
        DEFINE FIELD subscription_plan ON schools TYPE string DEFAULT 'starter' ASSERT $value INSIDE ['starter', 'basic', 'standard', 'premium', 'enterprise'];
        DEFINE FIELD subscription_expiry_date ON schools TYPE datetime;
        DEFINE FIELD subscription_status ON schools TYPE string DEFAULT 'trial' ASSERT $value INSIDE ['active', 'trial', 'expired', 'cancelled'];
        DEFINE FIELD created_at ON schools TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON schools TYPE datetime DEFAULT time::now();
        DEFINE INDEX school_name_idx ON schools FIELDS school_name;

        DEFINE TABLE users SCHEMAFULL;
        DEFINE FIELD school_id ON users TYPE record<schools> ASSERT $value != NONE;
        DEFINE FIELD user_type ON users TYPE string ASSERT $value != NONE AND $value INSIDE ['super_admin', 'admin', 'teacher', 'accountant', 'parent', 'student'];
        DEFINE FIELD first_name ON users TYPE string ASSERT $value != NONE;
        DEFINE FIELD last_name ON users TYPE string ASSERT $value != NONE;
        DEFINE FIELD email ON users TYPE string;
        DEFINE FIELD phone_number ON users TYPE string;
        DEFINE FIELD password_hash ON users TYPE string ASSERT $value != NONE;
        DEFINE FIELD status ON users TYPE string DEFAULT 'active' ASSERT $value INSIDE ['active', 'suspended', 'inactive'];
        DEFINE FIELD last_login ON users TYPE datetime;
        DEFINE FIELD email_verified ON users TYPE bool DEFAULT false;
        DEFINE FIELD phone_verified ON users TYPE bool DEFAULT false;
        DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now();
        DEFINE INDEX users_email_idx ON users FIELDS email UNIQUE;
        DEFINE INDEX users_school_type_idx ON users FIELDS school_id, user_type;
        DEFINE INDEX users_phone_idx ON users FIELDS phone_number;
        ").await?;

    DB.set(db).expect("Database already initialized");

    Ok(())
}

pub fn get_db() -> &'static Surreal<Client> {
    DB.get().expect("Database not initialized")
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

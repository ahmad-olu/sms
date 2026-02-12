// Copyright (C) 2026 Ahmad Olukotun
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.


use axum::{Json, Router, routing::get};
use shared::UserDto;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user", get(get_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_user() -> Json<UserDto> {
    Json(UserDto {
        id: 1,
        name: "Ahmad".into(),
    })
}

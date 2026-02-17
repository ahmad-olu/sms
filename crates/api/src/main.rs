// Copyright (C) 2026 Ahmad Olukotun
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

use axum::{Json, Router, response::Html, routing::get};
use listenfd::ListenFd;
use shared::UserDto;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .route("/", get(get_init))
        .route("/user", get(get_user));

    let mut listenfd = ListenFd::from_env();
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            tokio::net::TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap(),
    };

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_user() -> Json<UserDto> {
    info!("get_user called");
    Json(UserDto {
        id: 1,
        name: "Ahmad 2".into(),
    })
}

async fn get_init() -> Html<&'static str> {
    Html("<h1>Hello, World!!</h1>")
}

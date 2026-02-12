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

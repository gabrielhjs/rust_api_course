use std::io;

use crate::{database::Database, model::user::CreateUser};
use axum::{Router, extract::State, response::IntoResponse, routing};
use tokio::net::TcpListener;

mod database;
mod model;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> io::Result<()> {
  let app = Router::new()
    .route("/", routing::get(async || "Hello, World!"))
    .route("/user/create", routing::get(create_user))
    .route("/user", routing::get(list_user))
    .with_state(AppState::new().await);

  let listener = TcpListener::bind("127.0.0.1:8080").await?;

  axum::serve(listener, app).await
}

async fn create_user(
  State(AppState { database }): State<AppState>,
) -> impl IntoResponse {
  let new_user = CreateUser {
    name: "Gabriel".into(),
  };

  database.save(new_user).await;
}

async fn list_user(
  State(AppState { database }): State<AppState>,
) -> impl IntoResponse {
  let users = database.list().await;

  format!("{users:#?}")
}

#[cfg(test)]
mod test {
  use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
  };
  use tower::ServiceExt;

  use super::*;

  #[tokio::test]
  async fn test_list_user() {
    let state = AppState::new().await;

    let app = Router::new()
      .route("/", routing::get(list_user))
      .with_state(state);

    let req = Request::builder()
      .uri("/")
      .method(Method::GET)
      .body(Body::from(()))
      .unwrap();

    let resp = app
      .oneshot(req)
      .await
      .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
  }
}

use crate::model::user::User;
use sqlx::{QueryBuilder, Sqlite, migrate, sqlite::SqliteConnectOptions};

use super::Database;

#[derive(Clone)]
pub struct SqliteDatabase {
  pool: sqlx::Pool<Sqlite>,
}

impl SqliteDatabase {
  pub async fn new() -> Self {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
      .max_connections(5)
      .connect_with(
        SqliteConnectOptions::new()
          .filename("db.db")
          .create_if_missing(true),
      )
      .await
      .unwrap();

    migrate!()
      .run(&pool)
      .await
      .unwrap();

    Self { pool }
  }
}

impl Database<User> for SqliteDatabase {
  async fn save(&self, data: <User as crate::model::Model>::Create) {
    sqlx::query("INSERT INTO users(name) VALUES ($1)")
      .bind(data.name)
      .execute(&self.pool)
      .await
      .unwrap();
  }

  async fn get(&self, id: &<User as crate::model::Model>::Id) -> Option<User> {
    sqlx::query_as("SELECT * FROM users WHERE id = $1")
      .bind(id)
      .fetch_optional(&self.pool)
      .await
      .unwrap()
  }

  async fn update(
    &self,
    id: &<User as crate::model::Model>::Id,
    data: <User as crate::model::Model>::Update,
  ) -> Option<User> {
    let mut qb = QueryBuilder::new("UPDATE users SET ");
    let mut sep = qb.separated(", ");

    if let Some(name) = data.name {
      sep
        .push("name = ")
        .push_bind(name);
    }

    qb.push(" WHERE id = ")
      .push_bind(id);

    qb.build_query_as()
      .fetch_optional(&self.pool)
      .await
      .unwrap()
  }

  async fn delete(&self, id: &<User as crate::model::Model>::Id) {
    sqlx::query("DELETE users WHERE id = $1")
      .bind(id)
      .execute(&self.pool)
      .await
      .unwrap();
  }

  async fn list(&self) -> Vec<User> {
    sqlx::query_as("SELECT * FROM users")
      .fetch_all(&self.pool)
      .await
      .unwrap()
  }
}

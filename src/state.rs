#[cfg(all(feature = "sqlite"))]
mod state {
  use crate::database::sqlite_database::SqliteDatabase;

  #[derive(Clone)]
  pub struct AppState {
    pub database: SqliteDatabase,
  }

  impl AppState {
    pub async fn new() -> Self {
      Self {
        database: SqliteDatabase::new().await,
      }
    }
  }
}

#[cfg(not(feature = "sqlite"))]
mod state {
  use crate::database::in_memory_database::InMemoryDatabase;
  use std::sync::Arc;

  #[derive(Clone)]
  pub struct AppState {
    pub database: Arc<InMemoryDatabase>,
  }

  impl AppState {
    pub async fn new() -> Self {
      Self {
        database: Arc::new(InMemoryDatabase::new()),
      }
    }
  }
}

pub use self::state::*;

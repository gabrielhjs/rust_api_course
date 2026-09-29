use std::collections::HashMap;

use tokio::sync::RwLock;

use super::Database;
use crate::model::{Model, user::User};

pub struct InMemoryDatabase {
  users: RwLock<HashMap<<User as Model>::Id, User>>,
}

impl InMemoryDatabase {
  pub fn new() -> Self {
    Self {
      users: RwLock::new(HashMap::new()),
    }
  }
}

impl Database<User> for InMemoryDatabase {
  async fn save(&self, data: <User as Model>::Create) {
    let mut users = self.users.write().await;
    let id = users.len() as u32;

    users.insert(
      id,
      User {
        id,
        name: data.name,
      },
    );
  }

  async fn get(&self, id: &<User as Model>::Id) -> Option<User> {
    self
      .users
      .read()
      .await
      .get(id)
      .cloned()
  }

  async fn update(
    &self,
    id: &<User as Model>::Id,
    data: <User as Model>::Update,
  ) -> Option<User> {
    if let Some(mut user) = self
      .users
      .read()
      .await
      .get(id)
      .cloned()
    {
      if let Some(name) = data.name {
        user.name = name;
      }

      self
        .users
        .write()
        .await
        .insert(user.id, user.clone());

      Some(user)
    } else {
      None
    }
  }

  async fn delete(&self, id: &<User as Model>::Id) {
    self
      .users
      .write()
      .await
      .remove(id);
  }

  async fn list(&self) -> Vec<User> {
    self
      .users
      .read()
      .await
      .iter()
      .map(|(_, v)| v.clone())
      .collect::<Vec<User>>()
  }
}

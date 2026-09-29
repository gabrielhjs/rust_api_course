use super::Model;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "sqlite", derive(sqlx::FromRow))]
pub struct User {
  pub id: u32,
  pub name: String,
}

pub struct CreateUser {
  pub name: String,
}

pub struct UpdateUser {
  pub name: Option<String>,
}

impl Model for User {
  type Id = u32;
  type Create = CreateUser;
  type Update = UpdateUser;
}

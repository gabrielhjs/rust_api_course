use crate::model::Model;

#[cfg(not(feature = "sqlite"))]
pub mod in_memory_database;

#[cfg(feature = "sqlite")]
pub mod sqlite_database;

pub trait Database<M: Model> {
  fn save(&self, data: M::Create) -> impl Future<Output = ()>;
  fn get(&self, id: &M::Id) -> impl Future<Output = Option<M>>;
  fn update(
    &self,
    id: &M::Id,
    data: M::Update,
  ) -> impl Future<Output = Option<M>>;
  fn delete(&self, id: &M::Id) -> impl Future<Output = ()>;
  fn list(&self) -> impl Future<Output = Vec<M>>;
}

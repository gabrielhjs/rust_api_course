pub mod user;

pub trait Model: Clone {
  type Id;
  type Create;
  type Update;
}

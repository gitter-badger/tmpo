use std::collections::HashMap;

#[derive(Clone, serde::Serialize, Debug)]
pub struct Context {
  pub name: String,
  pub repository: Option<String>,
  pub username: Option<String>,
  pub email: Option<String>,
  pub values: HashMap<String, String>
}

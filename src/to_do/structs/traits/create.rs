use serde_json::{json, value::Value, Map};
// use text_colorizer::*;

use crate::state::write_to_file;

pub trait Create {
  fn create(
    &self,
    title: &String,
    status: &String,
    state: &mut Map<String, Value>,
    file_name: String,
  ) {
    state.insert(title.to_string(), json!(status));
    write_to_file(file_name, state);
    println!("\n\n{} is being created\n\n", title)
  }
}

use serde_json::{value::Value, Map};

use crate::state::write_to_file;

pub trait Delete {
  fn delete(&self, title: &String, state: &mut Map<String, Value>, file_name: &String) {
    state.remove(title);
    write_to_file(file_name, state);
    println!("\n\n{} is being deleted\n\n", title);
  }
}

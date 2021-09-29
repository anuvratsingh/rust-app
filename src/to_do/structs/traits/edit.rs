use serde_json::{Map, value::Value, json};

use crate::state::write_to_file;

pub trait Edit {
  fn set_to_done(&self, title: &String, state: &mut Map<String, Value>, file_name: String) {
    state.insert(title.to_string(), json!(String::from("done")));

    write_to_file(file_name, state);
    println!("\n\n{} is being set to done \n\n", title);
  }

  fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>, file_name: String) {
    state.insert(title.to_string(), json!(String::from("pending")));

    write_to_file(file_name, state);
    println!("\n\n{} is being set to pending \n\n", title);
  }
}
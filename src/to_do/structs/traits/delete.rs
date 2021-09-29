use serde_json::{value::Value, Map};

use crate::state::write_to_file;

/// Trait for deleting to-do item.
pub trait Delete {
  /// Deletes a to do item.
  ///
  /// # Arguments
  /// * title (&String): the title of the item to be deleted
  ///
  /// # Returns
  /// None
  fn delete(&self, title: &String, state: &mut Map<String, Value>, file_name: &String) {
    state.remove(title);
    write_to_file(file_name, state);
    println!("\n\n{} is being deleted\n\n", title);
  }
}

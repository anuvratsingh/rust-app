/// This struct defines the key attributes for a to-do struct.
/// 
/// # Attributes
/// * title (String): the title of the to-do item
/// * status (String): the status of the to-do item
pub struct Base {
  pub title: String,
  pub status: String,
}

impl Base {
  /// The constructor for the `Base` struct.
  /// 
  /// # Arguments
  /// * input_title (String): the titleof the to-do item
  /// * status (String): the status of the to-do item
  /// 
  /// # Returns
  /// (Base): the constructed `Base` struct
  pub fn new(input_title: &str, input_status: &str) -> Base {
    Base {
      title: input_title.to_string(),
      status: input_status.to_string(),
    }
  }
}

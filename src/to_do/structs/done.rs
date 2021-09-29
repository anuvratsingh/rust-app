use super::{
  base::Base,
  traits::{delete::Delete, edit::Edit, get::Get},
};

/// This struct defines a to do item for a done to do item.
///
/// # Attributes
/// * super_struct (Base): Inherited struct for housing key attributes
pub struct Done {
  pub super_struct: Base,
}

impl Done {
  /// The constructor for the Done struct.
  ///
  /// # Arguments
  /// * input_title (String): the title of the to do item
  ///
  /// # Returns
  /// (Done): the constructed Done struct
  pub fn new(input_title: &str) -> Done {
    let base: Base = Base::new(input_title, "done");

    Done { super_struct: base }
  }
}

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}

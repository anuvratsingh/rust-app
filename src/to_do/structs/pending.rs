use super::{
  base::Base,
  traits::{create::Create, delete::Delete, edit::Edit, get::Get},
};
/// This struct defines a to do item for a Pending to do item.
///
/// # Attributes
/// * super_struct (Base): Inherited struct for housing key attributes
pub struct Pending {
  pub super_struct: Base,
}

impl Pending {
  /// The constructor for the Done struct.
  ///
  /// # Arguments
  /// * input_title (String): the title of the to do item
  ///
  /// # Returns
  /// (Pending): the constructed Pending struct
  pub fn new(input_title: &str) -> Pending {
    let base: Base = Base::new(input_title, "pending");

    Pending { super_struct: base }
  }
}
impl Create for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
impl Get for Pending {}

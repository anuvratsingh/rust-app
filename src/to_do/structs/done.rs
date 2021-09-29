use super::{
  base::Base,
  traits::{delete::Delete, edit::Edit, get::Get},
};

pub struct Done {
  pub super_struct: Base,
}

impl Done {
  pub fn new(input_title: &str) -> Done {
    let input_status: String = String::from("done"); // Unused creation for no reason pg 51
    let base: Base = Base::new(input_title, "done");

    Done { super_struct: base }
  }
}

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}

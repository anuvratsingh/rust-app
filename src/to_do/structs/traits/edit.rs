pub trait Edit {
  fn set_to_done(&self, tittle: &str) {
    println!("{} is being set to done", tittle);
  }

  fn set_to_pending(&elf, tittle: &str) {
    println!("{} is being set to pending", tittle);
  }
}
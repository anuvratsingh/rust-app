pub trait Get {
  fn get(&self, tittle: &str) {
    println!("{} is being fetched", tittle);
  }
}
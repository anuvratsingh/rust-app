use serde_json::{json, value::Value, Map};

use std::{fs, fs::File, io::Read};

pub fn read_file(file_name: &String) -> Map<String, Value> {
  let mut file = File::open(file_name).unwrap();
  let mut data = String::new();
  file.read_to_string(&mut data).unwrap();
  let json: Value = serde_json::from_str(&data).unwrap();
  let state: Map<String, Value> = json.as_object().unwrap().clone();
  return state;
}

pub fn write_to_file(file_name: &String, state: &mut Map<String, Value>) {
  let new_data = json!(state);
  fs::write(file_name, new_data.to_string()).expect("ERROR: Unable to write file");
}

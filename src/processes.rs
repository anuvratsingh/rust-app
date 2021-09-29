use serde_json::{value::Value, Map};

use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;
use super::to_do::structs::traits::get::Get;
use super::to_do::ItemTypes;

fn processes_pending(
  item: Pending,
  command: String,
  state: &Map<String, Value>,
  file_name: &String,
) {
  let mut state = state.clone();
  match command.as_str() {
    "get" => item.get(&item.super_struct.title, &state),
    "create" => item.create(
      &item.super_struct.title,
      &item.super_struct.status,
      &mut state,
      file_name,
    ),
    "delete" => item.delete(&item.super_struct.title, &mut state, file_name),
    "edit" => item.set_to_done(&item.super_struct.title, &mut state, file_name),
    _ => println!("command {} not supported", command),
  }
}

fn processes_done(item: Done, command: String, state: &Map<String, Value>, file_name: &String) {
  let mut state = state.clone();
  match command.as_str() {
    "get" => item.get(&item.super_struct.title, &state),
    "delete" => item.delete(&item.super_struct.title, &mut state, file_name),
    "edit" => item.set_to_pending(&item.super_struct.title, &mut state, file_name),
    _ => println!("command: {} not supported", command),
  }
}

pub fn process_input(
  item: ItemTypes,
  command: String,
  state: &Map<String, Value>,
  file_name: &String,
) {
  match item {
    ItemTypes::Pending(item) => processes_pending(item, command, state, &file_name),
    ItemTypes::Done(item) => processes_done(item, command, state, &file_name),
  }
}

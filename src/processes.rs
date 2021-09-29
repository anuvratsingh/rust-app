use serde_json::{value::Value, Map};

use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;
use super::to_do::structs::traits::get::Get;
use super::to_do::ItemTypes;

/// This function processes the command on a pending to do item.
///
/// # item
/// * item (Pending): the to do item to be processed
/// * command (String): command to be acted on the to do item
/// * state (&Map<String, Value>): the state of the to do items for the program
/// * file_name (&String): name of the file on which action is being taken
///
/// # Returns
/// None
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


/// This function processes the command on a done to do item.
///
/// # item
/// * item (Pending): the to do item to be processed
/// * command (String): command to be acted on the to do item
/// * state (&Map<String, Value>): the state of the to do items for the program
/// * file_name (&String): name of the file on which action is being taken
///
/// # Returns
/// None
fn processes_done(item: Done, command: String, state: &Map<String, Value>, file_name: &String) {
  let mut state = state.clone();
  match command.as_str() {
    "get" => item.get(&item.super_struct.title, &state),
    "delete" => item.delete(&item.super_struct.title, &mut state, file_name),
    "edit" => item.set_to_pending(&item.super_struct.title, &mut state, file_name),
    _ => println!("command: {} not supported", command),
  }
}

/// This function processes the user input to decide which function to operate.
///
/// # Arguments
/// * item (ItemTypes): one of the to do item types to be processed
/// * command (String): the command to be acted on the to do item
/// * state (&Map<String, Value>): the state of the to do item for the program
/// * file_name (&String): name of the file on which action is being taken
/// # Returns
/// None
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

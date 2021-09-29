mod processes;
mod state;
mod to_do;
// External Crates
use serde_json::{value::Value, Map};
use std::env;

// Internal Crates
use processes::process_input;
use state::read_file;
use to_do::to_do_factory;



fn main() {
    // Create a Vec from arguments provided in cli
    let args: Vec<String> = env::args().collect();

    // Deinfe the parameters from the arguments
    let command: &String = &args[1];
    let title: &String = &args[2];

    // Name of the file being used
    let file_name: String = String::from("state.json");

    // Define outside the match scope so that it can be used later on
    let status: String;

    // Read the JSON file to get the saved to do items
    let state: Map<String, Value> = read_file(&file_name);

    // Check to see if the title is already there, setting status to pending if not
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = String::from("pending");
        }
    }
    // Create a to do struct depending on the status
    let item = to_do_factory(&status, title).expect(&status);

    // Affect the state based on the struct and command passed into the program
    process_input(item, command.to_string(), &state, &file_name);
}

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

// use text_colorizer::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let file_name: String = String::from("state.json");

    let status: String;

    let state: Map<String, Value> = read_file(file_name.clone());
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = String::from("pending");
        }
    }
    let item = to_do_factory(&status, title).expect(&status);
    process_input(item, command.to_string(), &state, file_name.clone());
}

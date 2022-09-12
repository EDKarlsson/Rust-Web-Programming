mod state;
mod to_do;
mod processes;

use serde_json::value::Value;
use serde_json::Map;
use state::{read_file, write_to_file};
use std::env;
use crate::processes::process_input;
use crate::to_do::to_do_factory;

#[allow(unused, unused_mut)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> = read_file(String::from("./state.json").as_str());
    println!("{:?}", state);
    let status: String;
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = "pending".to_string();
        }
    }
    let item = to_do_factory(&status, title)
        .expect(&status);
    process_input(item, command.to_string(), &state)
    // state.insert(title.to_string(), json!(status));
    // write_to_file("./state.json", &mut state);
}

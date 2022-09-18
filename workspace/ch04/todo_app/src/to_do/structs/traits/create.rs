use serde_json::{json, Map, Value};
use crate::state::write_to_file;

pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        println!("{} is being created", title);
        state.insert(title.to_string(), json!(status));
        write_to_file(String::from("./state.json"), state);
        println!("\n\n{} is being created\n\n", title);
    }
}
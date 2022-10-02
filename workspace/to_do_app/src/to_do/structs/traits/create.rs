use crate::state::write_to_file;
use serde_json::{json, Map, Value};

pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_to_file(String::from("./state.json"), state);
        println!("\n{} is being created\n", title);
    }
}

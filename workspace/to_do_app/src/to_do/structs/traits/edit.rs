use crate::state::write_to_file;
use serde_json::{json, Map, Value};

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        write_to_file(String::from("./state.json"), state);
        println!("\n{} is being set to done\n", title);
    }
    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("pending")));
        write_to_file(String::from("./state.json"), state);
        println!("\n{} is being set to pending\n", title);
    }
}

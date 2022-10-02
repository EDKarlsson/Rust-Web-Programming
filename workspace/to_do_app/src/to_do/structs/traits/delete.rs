use crate::state::write_to_file;
use serde_json::{Map, Value};

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file(String::from("./state.json"), state);
        println!("\n{} is being deleted\n", title);
    }
}

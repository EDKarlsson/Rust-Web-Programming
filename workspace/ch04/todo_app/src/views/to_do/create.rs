use actix_web::HttpRequest;
use serde_json::Map;
use serde_json::value::Value;

use crate::state::read_file;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file(String::from("./state.json"));
    let title: String = req.match_info().get("title").unwrap().to_string();
}
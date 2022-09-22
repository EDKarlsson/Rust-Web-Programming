use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::views::to_do::utils::return_state;
use actix_web::{web, HttpResponse};
use serde_json::{Map, Value};

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file(String::from("./state.json"));
    let title: String = to_do_item.title.clone();
    let status: String = to_do_item.status.clone();
    match to_do_factory(&status, title) {
        Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("delete"), &state),
    }
    HttpResponse::Ok().json(return_state())
}

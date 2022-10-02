use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use std::fmt;
use std::vec::Vec;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
            }
        }
        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;
        ToDoItems {
            pending_items: pending_array_buffer,
            done_item_count: done_count,
            pending_item_count: pending_count,
            done_items: done_array_buffer,
        }
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

impl fmt::Display for ToDoItems {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut print_string = String::new();

        if self.pending_item_count > 0 {
            print_string.push_str("pending_items=[");
            for pending_item in self.pending_items.iter() {
                print_string.push_str(pending_item.title.as_str());
                print_string.push(',');
            }
            print_string.push_str("]\n");
        }
        if self.done_item_count > 0 {
            print_string.push_str("done_items=[");
            for done_item in self.done_items.iter() {
                print_string.push_str(done_item.title.as_str());
                print_string.push(',');
            }
            print_string.push_str("]\n");
        }
        print_string.push_str(
            format!(
                "pending_item_count={},done_item_count={}",
                self.pending_item_count, self.done_item_count
            )
            .as_str(),
        );
        write!(f, "{}", print_string)
    }
}

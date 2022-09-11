use crate::to_do::structs::traits::create::Create;
use crate::to_do::{to_do_factory, ItemTypes};

mod to_do;

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "washing");
    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title
        )
    }
}
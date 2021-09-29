mod to_do;

use to_do::{structs::traits::create::Create, to_do_factory, ItemTypes};

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "washing");
    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!(
            "It's a done item with the title: {}",
            item.super_struct.title
        ),
    }
}

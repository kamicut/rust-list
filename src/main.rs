extern crate linkedlist;

fn main() {
    let mut list = linkedlist::List::new();
    list.prepend(3)
        .prepend(4);


    println!("{}, len: {}", list.stringify(), list.len());
}
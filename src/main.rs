extern crate linkedlist;

fn main() {
    let mut list = linkedlist::List::new();

    for i in 0..100 {
        list.prepend(i);
    }

    let tostring = list.iter()
        .map(|x| { x.to_string() })
        .collect::<Vec<String>>()
        .connect(", ");

    println!("{}", tostring);
    println!("length: {}", list.iter().count())
}
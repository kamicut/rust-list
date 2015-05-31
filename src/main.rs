use NodeType::{Node, Nil};
use std::fmt::Display;

type Link<T> = Box<NodeType<T>>;

#[derive(Clone)]
enum NodeType<T> where T:Clone+Display {
    Node(T, Link<T>),
    Nil,
}

struct List<T> where T:Clone+Display { start: Link<T> }

impl<T> List<T> where T:Clone+Display {
    pub fn new() -> List<T> {
        List { start: Box::new(Nil) }
    }

    fn prepend(&mut self, val: T) -> &mut List<T> {
        use std::mem::replace;
        let head = replace(&mut self.start, Box::new(Nil));
        self.start = Box::new(Node(val, head));
        self
    }

    fn iterate<F>(&self, mut f: F) where F: FnMut(Link<T>) {
       let mut ptr = self.start.clone();
       loop {
            f(ptr.clone());
            match *ptr {
                Nil => { break;  },
                Node(_, next) => { ptr = next; }
            }
       }
    }

    fn len(&self) -> usize {
        let mut ret_val = 0;
        self.iterate(|_ptr| {
           let ret = &mut ret_val;
           *ret += 1;
        });
        ret_val
    }

    fn stringify(&self) -> String {
        let mut ret_val = String::new();
        self.iterate(|ptr| {
            let ret = &mut ret_val;
            match *ptr {
                Nil => { *ret = format!("{}, Nil", *ret); },
                Node(val,_)=> {
                    if (*ret).len() > 0 {
                        *ret = format!("{}, {}", *ret, val);
                    } else {
                        *ret = format!("{}", val);
                    }
                },
            }
        });
        ret_val
    }
}

fn main() {
    let mut list = List::new();
    list.prepend(3).prepend(4);

    println!("{}, len: {}", list.stringify(), list.len());

}

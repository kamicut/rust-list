use std::fmt::Display;
use self::NodeType::{Node, Nil};

type Link<T> = Box<NodeType<T>>;

#[derive(Clone)]
enum NodeType<T> where T:Clone+Display {
    Node(T, Link<T>),
    Nil,
}

pub struct List<T> where T:Clone+Display { start: Link<T> }

impl<T> List<T> where T:Clone+Display {
    pub fn new() -> List<T> {
        List { start: Box::new(Nil) }
    }

    pub fn prepend(&mut self, val: T) -> &mut List<T> {
        use std::mem::replace;
        let head = replace(&mut self.start, Box::new(Nil));
        self.start = Box::new(Node(val, head));
        self
    }

    fn iterate<F>(&self, mut f: F) where F: FnMut(&Link<T>) {
       let mut ptr = self.start.clone();
       loop {
            f(&ptr);
            match *ptr {
                Nil => { break;  },
                Node(_, next) => { ptr = next; }
            }
       }
    }

    pub fn len(&self) -> usize {
        let mut ret_val = 0;
        self.iterate(|_ptr| {
           let ret = &mut ret_val;
           *ret += 1;
        });
        ret_val
    }

    pub fn stringify(&self) -> String {
        let mut ret_val = String::new();
        self.iterate(|ptr| {
            let ret = &mut ret_val;
            match **ptr {
                Nil => { *ret = format!("{}, Nil", *ret); },
                Node(ref val,_)=> {
                    if (*ret).len() > 0 {
                        *ret = format!("{}, {}", *ret, *val);
                    } else {
                        *ret = format!("{}", *val);
                    }
                },
            }
        });
        ret_val
    }
}

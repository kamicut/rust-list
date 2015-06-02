use std::fmt::Display;
use self::NodeType::{Node, Nil};

type Link<T> = Box<NodeType<T>>;

#[derive(Clone)]
enum NodeType<T> where T:Clone+Display {
    Node(T, Link<T>),
    Nil,
}

pub struct List<T> where T:Clone+Display {
    start: Link<T>,
}

pub struct ListIter<'a, T: 'a> where T: Clone+Display{
    current: &'a Link<T>
}

impl<'a, T> Iterator for ListIter<'a, T>
 where T: Clone+Display {
    type Item=T;

    fn next(&mut self) -> Option<T> {
        match **self.current {
            Nil => { None },
            Node(ref val, ref next) => {
                self.current = next;
                Some((*val).clone())
            }
        }
    }
}

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

    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: &self.start
        }
    }
}

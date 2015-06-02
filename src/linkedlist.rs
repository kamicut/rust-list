use self::NodeType::{Node, Nil};

type Link<T> = Box<NodeType<T>>;

enum NodeType<T> {
    Node(T, Link<T>),
    Nil,
}

pub struct List<T> { head: Link<T>, }

pub struct ListIter<'a, T: 'a> { current: &'a Link<T> }

impl<'a, T> Iterator for ListIter<'a, T> where T: Clone {
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

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: Box::new(Nil) }
    }

    pub fn prepend(&mut self, val: T) -> &mut List<T> {
        use std::mem::replace;
        let head = replace(&mut self.head, Box::new(Nil));
        self.head = Box::new(Node(val, head));
        self
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: &self.head
        }
    }
}

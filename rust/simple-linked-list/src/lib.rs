use std::iter::FromIterator;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        Self::len_imp(&self.head)
    }
    fn len_imp(node: &Option<Box<Node<T>>>) -> usize {
        match node {
            None => 0,
            Some(n) => Self::len_imp(&n.next) + 1,
        }
    }

    pub fn push(&mut self, element: T) {
        match self.head {
            None => {
                self.head = Some(Box::new(Node {
                    data: element,
                    next: None,
                }))
            }
            Some(_) => {
                let mut n = Self::push_imp(&mut self.head);
                n.next = Some(Box::new(Node {
                    data: element,
                    next: None,
                }))
            }
        }
    }

    fn push_imp(node: &mut Option<Box<Node<T>>>) -> &mut Node<T> {
        match node {
            None => panic!("hoge"),
            Some(n) => match n.next {
                None => n,
                Some(_) => Self::push_imp(&mut n.next),
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self { head: None } => None,
            Self { head: Some(_) } => {
                let node = self.head.take().unwrap();
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
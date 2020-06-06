use std::iter::FromIterator;
#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Copy + PartialOrd + std::fmt::Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut hn = &self.head;
        let mut length = 0;
        while hn.is_some() {
            length += 1;
            hn = &hn.as_ref().unwrap().next;
        }
        length
    }

    pub fn push(&mut self, element: T) {
        match self {
            Self { head: None } => {
                self.head = Some(Box::new(Node {
                    data: element,
                    next: None,
                }));
            }
            Self { head: Some(_) } => {
                let node = self.head.take().unwrap();
                self.head = Some(Box::new(Node {
                    data: element,
                    next: Some(node),
                }));
            }
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
        match self {
            Self { head: None } => None,
            Self { head: Some(node) } => Self::peek_imp(&self.head, &node.data),
        }
    }
    fn peek_imp<'a>(node: &'a Option<Box<Node<T>>>, max: &'a T) -> Option<&'a T> {
        match node {
            None => Some(max),
            Some(cur_node) => {
                let new_max = if *max < cur_node.data {
                    &cur_node.data
                } else {
                    max
                };
                Self::peek_imp(&cur_node.next, new_max)
            }
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut vec: Vec<T> = self.into();
        vec.reverse();
        vec.into_iter().collect::<SimpleLinkedList<T>>()
    }
}

impl<T: Copy + PartialOrd + std::fmt::Debug> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in iter {
            list.push(i);
        }
        return list;
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
        fn into_imp<T>(node: Option<Box<Node<T>>>, mut v: Vec<T>) -> Vec<T> {
            match node {
                None => v,
                Some(cur_node) => {
                    v.push(cur_node.data);
                    into_imp(cur_node.next, v)
                }
            }
        }
        let v = Vec::new();
        let mut v2 = match self {
            Self { head: None } => v,
            Self { head: Some(_) } => into_imp(self.head, v),
        };
        v2.reverse();
        v2
    }
}

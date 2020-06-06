use std::iter::FromIterator;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Clone + PartialOrd> SimpleLinkedList<T> {
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
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
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
        match self.head {
            None => None,
            Some(_) => Some(&self.head.as_ref().unwrap().data),
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        let mut np = &(self.head);
        while np.is_some() {
            list.push(np.as_ref().unwrap().data.clone());
            np = &np.as_ref().unwrap().next;
        }
        list
    }
}

impl<T: Copy + PartialOrd> FromIterator<T> for SimpleLinkedList<T> {
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

impl<T: Copy + PartialOrd + std::fmt::Debug> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = vec![];
        while let Some(elem) = self.pop() {
            v.push(elem);
        }
        v.into_iter().rev().collect()
    }
}

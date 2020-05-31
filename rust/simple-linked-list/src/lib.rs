use std::iter::FromIterator;

//pub struct Entry<T> {
//    value: T,
//    next: Box<Entry<T>>,
//}
//
//pub enum List<T> {
//    Nil,
//    Entry(T),
//}

//pub struct Entry<T> {
//    value: T,
//    next: NextPointer<T>,
//}
//
//pub enum NextPointer<T> {
//    Nil,
//    Box<Entry<T>>,
//}

pub struct Entry<T> {
    value: T,
    next: Box<NextPointer<T>>,
}

pub enum NextPointer<T> {
    Nil,
    Elem(Entry<T>),
}

pub struct SimpleLinkedList<T> {
    first: Box<NextPointer<T>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            first: Box::new(NextPointer::Nil),
        }
    }

    pub fn len(&self) -> usize {
        match *self.first {
            NextPointer::Nil => 0,
            _ => Self::len_imp(&self.first),
        }
    }
    fn len_imp(p: &Box<NextPointer<T>>) -> usize {
        match &**p {
            NextPointer::Nil => 0,
            NextPointer::Elem(Entry { next, .. }) => Self::len_imp(&next) + 1,
        }
    }

    pub fn push(&mut self, element: T) {
        match *self.first {
            NextPointer::Nil => {
                self.first = Box::new(NextPointer::Elem(Entry {
                    value: element,
                    next: Box::new(NextPointer::Nil),
                }));
            }
            _ => {
                let mut entry = Self::last(&mut self.first);
                entry.next = Box::new(NextPointer::Elem(Entry {
                    value: element,
                    next: Box::new(NextPointer::Nil),
                }));
            }
        }
    }
    fn last(p: &mut Box<NextPointer<T>>) -> &mut Entry<T> {
        match &&(**p) {
            NextPointer::Nil => panic!("bug"),
            NextPointer::Elem(&entry) => match *entry.next {
                NextPointer::Nil => return &mut entry,
                NextPointer::Elem(Entry { next: next, .. }) => Self::last(&mut next),
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
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

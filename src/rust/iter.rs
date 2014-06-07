use rust::option::{Option, Some, None};

pub trait Iterator<A> {
    fn next(&mut self) -> Option<A>;

    #[inline]
    #[no_split_stack]
    fn zip<U, B: Iterator<U>>(self, other: B) -> Zip<Self, B> {
        Zip { a: self, b: other }
    }
}

pub struct Zip<T, U> {
    a: T,
    b: U,
}

impl<A, B, T: Iterator<A>, U: Iterator<B>> Iterator<(A, B)> for Zip<T, U> {
    #[inline]
    #[no_split_stack]
    fn next(&mut self) -> Option<(A, B)> {
        match self.a.next() {
            Some(a) => match self.b.next() {
                Some(b) => Some((a, b)),
                _ => None,
            },
            _ => None,
        }
    }
}

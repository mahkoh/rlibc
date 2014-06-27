use rust::option::*;
use rust::lang::*;
use rust::clone::{Clone};
use rust::num::{One};

pub trait Iterator<A> {
    fn next(&mut self) -> Option<A>;

    #[inline]
    #[no_split_stack]
    fn zip<U, B: Iterator<U>>(self, other: B) -> Zip<Self, B> {
        Zip { a: self, b: other }
    }

    #[inline]
    #[no_split_stack]
    fn any(&mut self, f: |A| -> bool) -> bool {
        for a in *self {
            if f(a) {
                return true;
            }
        }
        false
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

pub struct LoopFrom<A> {
    state: A,
    one: A,
}

#[inline]
pub fn loop_from<A: Add<A,A>+PartialOrd+Clone+One>(start: A) -> LoopFrom<A> {
    LoopFrom{state: start, one: One::one()}
}

impl<A: Add<A,A>+PartialOrd+Clone+One> Iterator<A> for LoopFrom<A> {
    #[inline]
    fn next(&mut self) -> Option<A> {
        let result = self.state.clone();
        self.state = self.state + self.one;
        Some(result)
    }
}

pub struct Range<A> {
    state: A,
    stop: A,
    one: A
}

#[inline]
pub fn range<A: Add<A,A>+PartialOrd+Clone+One>(start: A, stop: A) -> Range<A> {
    Range{state: start, stop: stop, one: One::one()}
}

impl<A: Add<A,A>+PartialOrd+Clone+One> Iterator<A> for Range<A> {
    #[inline]
    fn next(&mut self) -> Option<A> {
        if self.state < self.stop {
            let result = self.state.clone();
            self.state = self.state + self.one;
            Some(result)
        } else {
            None
        }
    }
}

impl<A: Add<A,A>+PartialOrd+Clone+One+Sub<A,A>> DoubleEndedIterator<A> for Range<A> {
    #[inline]
    fn next_back(&mut self) -> Option<A> {
        if self.stop > self.state {
            self.stop = self.stop - self.one;
            Some(self.stop.clone())
        } else {
            None
        }
    }
}

pub trait DoubleEndedIterator<A>: Iterator<A> {
    fn next_back(&mut self) -> Option<A>;

    #[inline]
    fn rev(self) -> Rev<Self> {
        Rev{iter: self}
    }
}

pub struct Rev<T> {
    iter: T
}

impl<A, T: DoubleEndedIterator<A>> Iterator<A> for Rev<T> {
    #[inline]
    fn next(&mut self) -> Option<A> { self.iter.next_back() }
}

impl<A, T: DoubleEndedIterator<A>> DoubleEndedIterator<A> for Rev<T> {
    #[inline]
    fn next_back(&mut self) -> Option<A> { self.iter.next() }
}

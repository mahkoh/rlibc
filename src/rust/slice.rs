use rust::iter::{Iterator};
use rust::option::{Option, None, Some};
use rust::{offset, transmute, Repr};

use syscalls::{sys_exit};

pub struct Slice<T> {
    pub data: *T,
    pub len: uint,
}

impl<'a, T> Repr<Slice<T>> for &'a [T] { }

pub trait ToSlice<T> {
    fn to_slice(self, len: uint) -> &[T];
}

impl<T> ToSlice<T> for *mut T {
    #[inline]
    #[no_split_stack]
    fn to_slice(self, len: uint) -> &[T] {
        new_slice(self as *T, len)
    }
}

impl<T> ToSlice<T> for *T {
    #[inline]
    #[no_split_stack]
    fn to_slice(self, len: uint) -> &[T] {
        new_slice(self, len)
    }
}

#[inline]
#[no_split_stack]
fn new_slice<T>(p: *T, len: uint) -> &[T] {
    unsafe { 
        transmute(Slice {
            data: p,
            len: len
        })
    }
}

pub trait ImmutableSlice<'a, T> {
    fn slice(&self, start: uint, end: uint) -> &'a [T];
    fn len(&self) -> uint;
    fn raw(&self) -> *T;
    fn lastn(&self, n: uint) -> &'a [T];
    fn iter(&'a self) -> Items<'a, T>;
}

impl<'a, T> ImmutableSlice<'a, T> for &'a [T] {
    #[inline]
    #[no_split_stack]
    fn slice(&self, start: uint, end: uint) -> &'a [T] {
        if start > end || end > self.len() {
            unsafe { sys_exit(134); }
        }
        unsafe { new_slice(offset(self.raw(), start as int), end-start) }
    }

    #[inline]
    #[no_split_stack]
    fn len(&self) -> uint {
        self.repr().len
    }

    #[inline]
    #[no_split_stack]
    fn raw(&self) -> *T {
        self.repr().data
    }

    #[inline]
    #[no_split_stack]
    fn lastn(&self, n: uint) -> &'a [T] {
        if self.len() < n {
            unsafe { sys_exit(134); }
        }
        unsafe { new_slice(offset(self.raw(), (self.len()-n) as int), n) }
    }

    #[inline]
    #[no_split_stack]
    fn iter(&self) -> Items<'a, T> {
        let raw = self.repr();
        unsafe { Items { pos: raw.data, end: offset(raw.data, raw.len as int) } }
    }
}

pub struct Items<'a, T> {
    pos: *T,
    end: *T,
}

impl<'a, T> Iterator<&'a T> for Items<'a, T> {
    #[inline]
    #[no_split_stack]
    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            if self.pos == self.end {
                None
            } else {
                let old = self.pos;
                self.pos = offset(self.pos, 1);
                Some(transmute(old))
            }
        }
    }
}

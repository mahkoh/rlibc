#![no_std]
#![crate_id="rlibc"]
#![crate_type="staticlib"]
#![allow(non_camel_case_types)]
#![allow(visible_private_types)]
#![allow(non_snake_case_functions)]
#![allow(dead_code)]
#![feature(asm, globs, macro_rules)]

pub use rust::x86_64::linux::start::_start;

mod rust;

mod types;

pub mod libc;
pub mod posix;
pub mod syscalls;

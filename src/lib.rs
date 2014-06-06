#![no_std]
#![crate_id="rlibc"]
#![crate_type="staticlib"]
#![allow(non_camel_case_types)]
#![allow(visible_private_types)]
#![feature(asm, globs, macro_rules)]

pub mod internal;

mod types;

pub mod libc;
pub mod syscalls;

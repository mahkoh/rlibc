#![no_std]
#![crate_id="rlibc"]
#![crate_type="staticlib"]
#![allow(non_camel_case_types)]
#![allow(visible_private_types)]
#![allow(non_snake_case_functions)]
#![allow(dead_code)]
#![feature(asm, globs, macro_rules, lang_items, intrinsics)]

extern crate core;

pub use rust::x86_64::linux::start::_start;

mod rust;

mod types;
mod consts;

pub mod libc;
pub mod posix;
pub mod syscalls;

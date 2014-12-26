#![no_std]
#![crate_name="rlibc"]
#![crate_type="staticlib"]
#![allow(non_camel_case_types)]
#![allow(visible_private_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(asm, globs, macro_rules, lang_items, intrinsics)]

extern crate core;

#[cfg(any(all(target_os = "linux", target_arch = "x86_64"),
		  all(target_os = "android", target_arch = "x86_64")))]
pub use rust::x86_64::linux::start::_start;

#[cfg(any(all(target_os = "macos", target_arch = "x86_64"),
		  all(target_os = "ios", target_arch = "x86_64")))]
pub use rust::x86_64::macos::start::start;

mod rust;

mod types;
mod consts;

pub mod libc;
pub mod posix;
pub mod math;
pub mod syscalls;

// WARNING hacks
#[lang = "stack_exhausted"] extern fn stack_exhausted() {
	unsafe {syscalls::sys_exit(1);}
}
#[lang = "eh_personality"] extern fn eh_personality() {
	unsafe {syscalls::sys_exit(1);}
}
#[lang = "panic_fmt"]
unsafe fn panic_fmt() -> ! {
	syscalls::sys_exit(1);
	loop { };
}
#[lang = "begin_unwind"]
unsafe extern "C" fn begin_unwind(
		_fmt: &core::fmt::Arguments,
		_file: &str,
		_line: uint
	) -> ! {
	syscalls::sys_exit(1);
    loop { }; // for divergence check
}

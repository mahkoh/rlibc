use rust::prelude::*;
use types::{int_t, void_t, pid_t};
use syscalls::{sys_kill,};
use libc::errno::errno;
use posix::stdlib::{_exit,};
use posix::unistd::{getpid,};

pub use self::Signals::*;

pub type sighandler_t = fn(int_t);

type __sa_handler_t = fn(int_t);
type __sa_sigaction = fn(int_t, *mut void_t, *mut void_t);
type __sigaction_u_t = *mut void_t;
type sigset_t = u32;
#[repr = "C"]
struct sigaction_s {
	__sigaction_u: __sigaction_u_t,
	sa_mask: sigset_t,
	sa_flags: int_t,
}

macro_rules! forward {
    ($sys:ident, $($p:expr),*) => {
        match $sys($($p),+) {
            n if n < 0 => {
                errno = -n;
                -1
            },
            n => n,
        }
    };
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub mod Signals {
	use types::{int_t,};

	/// Signal Interrupt: Interactive attention signal.
	pub const SIGINT: int_t = 2;
	/// Signal Illegal Instruction: Invalid function image.
	pub const SIGILL: int_t = 4;
	/// Signal Abort: Abnormal termination.
	pub const SIGABRT: int_t = 6;
	/// Signal Floating-Point Exception: Erroneous arithmetic operation.
	pub const SIGFPE: int_t = 8;
	/// Signal Segmentation Violation: Invalid access to memory storage.
	pub const SIGSEGV: int_t = 11;
	/// Signal Terminate: Termination request sent to program.
	pub const SIGTERM: int_t = 15;
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub mod Signals {
	use types::{int_t,};

	/// Signal Interrupt: Interactive attention signal.
	pub const SIGINT: int_t = 2;
	/// Signal Illegal Instruction: Invalid function image.
	pub const SIGILL: int_t = 4;
	/// Signal Abort: Abnormal termination.
	pub const SIGABRT: int_t = 6;
	/// Signal Floating-Point Exception: Erroneous arithmetic operation.
	pub const SIGFPE: int_t = 8;
	/// Signal Segmentation Violation: Invalid access to memory storage.
	pub const SIGSEGV: int_t = 11;
	/// Signal Terminate: Termination request sent to program.
	pub const SIGTERM: int_t = 15;
}

/// Generates a signal
/// Sends signal `sig` to the current executing program.
/// The signal is handled as specified by function `signal`.
#[no_mangle]
pub unsafe extern fn raise(sig: int_t) -> int_t {
	kill(getpid(), sig)
}

/// Set function to handle signal.
/// Specifies a way to handle the signals with the signal number specified by sig.
#[no_mangle]
pub unsafe extern fn signal(sig: int_t,
							func: sighandler_t) -> sighandler_t {
	_exit(1); // TODO implement signal attachment
}

/// Send a signal to a process or a group of processes.
#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern fn kill(pid: pid_t, sig: int_t) -> int_t {
	forward!(sys_kill, pid, sig)
}
#[no_mangle]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub unsafe extern fn kill(pid: pid_t, sig: int_t) -> int_t {
	forward!(sys_kill, pid as int_t, sig, 0)
}

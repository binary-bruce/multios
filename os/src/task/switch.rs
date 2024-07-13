//! Rust wrapper around `__switch`.
//!
//! Switching to a different task's context happens here. The actual
//! implementation must not be in Rust and (essentially) has to be in assembly
//! language (Do you know why?), so this module really is just a wrapper around
//! `switch.S`.

use core::arch::global_asm;

use super::TaskContext;

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(current_cx: *mut TaskContext, next_cx: *mut TaskContext);
}

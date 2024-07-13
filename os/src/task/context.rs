//! Implementation of [`TaskContext`]

/// TaskContext
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TaskContext {
    /// return address (e.g. __restore) of __switch function
    ra: usize,

    /// kernel stack pointer of app
    sp: usize,

    /// callee saved registers: s 0..11
    s: [usize; 12],
}

impl TaskContext {
    /// init task context
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    /// set task context
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }

        println!("kernel: task context, ra {}, sp {}", __restore as usize, kstack_ptr);

        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}

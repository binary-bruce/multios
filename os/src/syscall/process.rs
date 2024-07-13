//! Process management syscalls

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    panic!("Unreachable in sys_exit!");
}

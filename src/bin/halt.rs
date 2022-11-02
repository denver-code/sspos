#![no_std]
#![no_main]

use sspos::api::syscall;
use sspos::entry_point;

entry_point!(main);

fn main(_args: &[&str]) {
    syscall::write(1, b"\x1b[93m"); // Yellow
    syscall::write(1, b"sspos has reached its fate, the system is now halting.\n");
    syscall::write(1, b"\x1b[0m"); // Reset
    syscall::sleep(0.5);
    syscall::halt();
    loop { syscall::sleep(1.0) }
}

// Triggers the out-of-memory handler. Should print an error message.

#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec::Vec;
use core::alloc::Layout;
use core::fmt::Write;
use libtock::result::TockResult;
use libtock::syscalls;

#[libtock::main]
fn main() -> TockResult<()> {
    let mut vec = Vec::new();
    loop {
        vec.push(0);
    }
}

#[alloc_error_handler]
unsafe fn alloc_error_handler(_: Layout) -> ! {
    if let Ok(drivers) = libtock::retrieve_drivers() {
        let mut console = drivers.console.create_console();
        let _ = writeln!(console, "alloc_error_handler called");
    }
    loop {
        syscalls::raw::yieldk();
    }
}

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(qemu_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use vga_buffer::println;

// The entry point for the OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop{}
}

#[test_case]
fn test_println() {
    println!("test println output");
}
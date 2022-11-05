#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

mod vga_buffer;

// The entry point for the OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Some numbers: {} {}", 42, 1.337);
    println!("I'm on the next line now! ");
    println!("This line is so looooooooooooooooooooooooooooooooooooooong it wraps onto the next one! ");
    panic!("Some panic message");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(qemu_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(not(test))]
use core::panic::PanicInfo;
use vga_buffer::println;
use vga_buffer::Color;
use vga_buffer::WRITER;

// The entry point for the OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("Yabba dabba doo!");
    WRITER.lock().set_color(Color::Black, Color::White);
    println!("This should be a different colour now");
    panic!("Test panic");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

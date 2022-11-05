#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// The entry point for the OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello World!").unwrap();
    write!(vga_buffer::WRITER.lock(), " Some numbers: {} {} \n", 42, 1.337).unwrap();
    write!(vga_buffer::WRITER.lock(), "I'm on the next line now! \n").unwrap();
    write!(vga_buffer::WRITER.lock(), "This line is so looooooooooooooooooooooooooooooooooooooong it wraps onto the next one! \n").unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

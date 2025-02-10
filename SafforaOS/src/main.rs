#![no_std] // no standard library
#![no_main] // no main function

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} } // panic handler

static HELLO: &[u8] = b"Hello, world!\n";

#[no_mangle]
pub extern "C" fn _start() -> ! { // defined entry point "_start"
    let vga_buffer = 0xb8000 as *mut u8; // VGA buffer address

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // character
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color
        }
    }

    loop {}
}
 
#![no_std]      // No standard library is linked 
#![no_main]     // disable all Rust-level entry points
use core::panic::PanicInfo;

// a static variable that holds the string to be printed
static HELLO: &[u8] = b"Hello World!";

// custom entry point for the program ,since the linker looks for a function
// named `_start` by default
#[unsafe(no_mangle)] // it doest change the name of the function in the compiled binary
pub extern "C" fn _start() -> ! {

    // vga buffer is located at 0xb8000 in memory
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}

// compiler will call this function on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


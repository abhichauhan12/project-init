 
#![no_std]      // No standard library is linked 
#![no_main]     // disable all Rust-level entry points
use core::panic::PanicInfo;

// compiler will call this function on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// custom entry point for the program
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop{}
}
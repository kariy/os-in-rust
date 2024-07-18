#![no_std] // don't link the rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

/// The function that the compiler should invoke when a panic occurs.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// The `#[no_mangle]` attribute is used to prevent the Rust compiler from "[mangling]" the name of
/// this external function. Without the attribute, the compiler would generate some cryptic
/// _ZN3blog_os4_start7hb173fedf945531caE symbol to give every function a unique name.
///
/// [mangling]: https://en.wikipedia.org/wiki/Name_mangling
///
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

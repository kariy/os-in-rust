#![no_std] // don't link the rust standard library
#![no_main] // disable all Rust-level entry points

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
    // cast the integer into a raw mutable pointer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in b"Hello World!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // ASCII byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color byte (light cyan)
        }
    }

    loop {}
}

#![no_std]
#![no_main]

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
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by defaul
    loop {}
}

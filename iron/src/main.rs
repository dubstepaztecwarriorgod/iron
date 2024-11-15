#![no_std]
#![no_main]

// pub use core as core;

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn _start() -> ! {
    loop {}
}

// #[no_mangle]
// fn entry_32_bit() -> ! {
//     loop {}
// }

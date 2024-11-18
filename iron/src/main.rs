#![no_std]
#![no_main]

// pub use core as core;

mod vga;

use core::fmt::{Debug, Pointer, Write};
use core::panic::PanicInfo;
use bootloader::BootInfo;
use bootloader::entry_point;
use ufmt::{uWrite, uwriteln};
use ufmt::uwrite;
use vga::VgaWriter;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let mut vga = VgaWriter::new();
    vga.clear();
    let mut formatter = ufmt::Formatter::new(&mut vga);
    let _ = formatter.write_str("Kernel Panic: ");

    let message = panic_info.message();
    if let Some(m) = message.as_str() {
        uwrite!(&mut formatter, "'{}'", m).unwrap();
    } else {
        let _ = formatter.write_str("unknown panic message");
    }

    if let Some(location) = panic_info.location() {
        let file = location.file();
        let line = location.line();
        let column = location.column();
        uwrite!(&mut formatter,"{}:{} {}", file, line, column);
    } else {
        uwrite!(&mut formatter, "Unknown panic location");
    }

    halt();
}


// burn some cpu cycles. Around a second in QEMU
fn delay(count: usize) {
    for i in 0..count* 7_000_000 {}
}


fn kernel(boot_info: &'static BootInfo) {
    let mut vga = VgaWriter::new();

    vga.clear();
    delay(1);
    vga.flood('a');
    delay(1);
    vga.flood('b');
    delay(1);
    vga.clear();

    panic!("meow!");
}

entry_point!(kernel_start);

#[no_mangle]
fn kernel_start(boot_info: &'static BootInfo) -> ! {
    kernel(boot_info);
    halt();
}

fn halt() -> ! {
    loop {}
}

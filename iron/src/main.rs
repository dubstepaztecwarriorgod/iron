#![no_std]
#![no_main]

// pub use core as core;


use core::fmt::{Arguments, Debug, Pointer, Write};
use core::panic::PanicInfo;
use bootloader::BootInfo;
use bootloader::entry_point;
use ufmt::uWrite;
use ufmt::uwrite;
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let mut vga = Vga::new();
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




struct Vga {
    buffer_index: usize,
}

// A simple wrapping vga buffer writer
impl Vga {
    const BUFFER_HEIGHT: usize = 25;
    const BUFFER_WIDTH: usize = 80;
    fn new() -> Self {
        Vga { buffer_index: 0 }
    }

    fn push_char(&mut self, c: char) {
        if c == '\n' {
            let row =  self.buffer_index / Self::BUFFER_WIDTH as usize;
            let row = row + 1;
            self.buffer_index = row * Self::BUFFER_WIDTH;
            return;
        }

        let max_chars = (Self::BUFFER_HEIGHT * Self::BUFFER_WIDTH);
        if self.buffer_index >= max_chars {
            self.buffer_index = 0;
        }

        let vga_start = (0xb8000 as *mut u8);
        let color_white = 15;
        unsafe {
            vga_start.offset((self.buffer_index*2) as isize).write_volatile(c as u8);
            vga_start.offset((self.buffer_index*2+1) as isize).write_volatile(color_white);
        }

        self.buffer_index += 1;
    }

    fn clear(&mut self) {
        for _ in 0..Self::BUFFER_HEIGHT* Self::BUFFER_WIDTH {
            self.push_char(' ');
        }
    }

    fn flood(&mut self, c: char) {
        for _ in 0..Self::BUFFER_WIDTH * Self::BUFFER_HEIGHT {
            self.push_char(c);
        }
    }

    fn push_str(&mut self, string: &str) {
        for c in string.chars() {
            self.push_char(c);
        }
    }
}

impl uWrite for Vga {
    type Error = ();


    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.push_str(s);
        return Ok(());
    }

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.push_char(c);
        Ok(())
    }
}

// burn some cpu cycles. Around a second in QEMU
fn delay(count: usize) {
    for i in 0..count* 7_000_000 {}
}

fn kernel() {
    let mut vga = Vga::new();
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
fn kernel_start(a: &BootInfo) -> ! {
    kernel();
    halt();
}

fn halt() -> ! {
    loop {}
}

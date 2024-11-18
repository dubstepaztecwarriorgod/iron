use ufmt::uWrite;

pub struct VgaWriter {
    buffer_index: usize,
}

// A simple wrapping vga buffer writer
impl VgaWriter {
    const BUFFER_HEIGHT: usize = 25;
    const BUFFER_WIDTH: usize = 80;
    pub fn new() -> Self {
        VgaWriter { buffer_index: 0 }
    }

    pub fn push_char(&mut self, c: char) {
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

    pub fn clear(&mut self) {
        for _ in 0..Self::BUFFER_HEIGHT* Self::BUFFER_WIDTH {
            self.push_char(' ');
        }
    }

    pub fn flood(&mut self, c: char) {
        for _ in 0..Self::BUFFER_WIDTH * Self::BUFFER_HEIGHT {
            self.push_char(c);
        }
    }

    pub fn push_str(&mut self, string: &str) {
        for c in string.chars() {
            self.push_char(c);
        }
    }
}

impl uWrite for VgaWriter {
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
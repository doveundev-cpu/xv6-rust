use core::ptr::write_volatile;

pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub const fn new(addr: usize) -> Self {
        Self { base_address: addr }
    }

    pub fn init(&self) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            // Thiết lập word length là 8 bits (LCR thanh ghi)
            write_volatile(ptr.add(3), 0b11);
            // Bật FIFO (FCR thanh ghi)
            write_volatile(ptr.add(2), 0b1);
            // Bật ngắt nhận dữ liệu (IER thanh ghi)
            write_volatile(ptr.add(1), 0b1);
        }
    }

    pub fn putc(&self, c: u8) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            write_volatile(ptr, c);
        }
    }
}

// Implement trait Write để dùng được macro write! sau này
impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.putc(c);
        }
        Ok(())
    }
}

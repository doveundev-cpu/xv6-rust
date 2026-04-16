#![no_std]
#![no_main]

mod uart;

use core::fmt::Write;
use core::panic::PanicInfo;
use uart::Uart;

/// Hàm này sẽ được gọi khi code xảy ra lỗi (panic).
/// Vì không có hệ điều hành, chúng ta chỉ có thể lặp vô tận.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Điểm nhập của hệ điều hành.
/// `export_name` đảm bảo linker tìm thấy ký hiệu này.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Sau này code khởi tạo UART và Kernel sẽ nằm ở đây
    let mut uart = Uart::new(0x10000000);
    uart.init();

    let _ = write!(uart, "Hello, xv6-rust!\n");

    loop {
        // Sử dụng inline assembly để tạm dừng CPU tiết kiệm điện
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

#![no_std]
#![no_main]

mod kalloc;
mod uart;
mod vm;
use core::fmt::Write;
use core::panic::PanicInfo;
use kalloc::{PageAllocator, PAGE_SIZE};
use uart::Uart;

// Khai báo ký hiệu từ Linker
extern "C" {
    static _end: u8;
}

static mut ALLOCATOR: PageAllocator = PageAllocator::new();

/// Hàm này sẽ được gọi khi code xảy ra lỗi (panic).
/// Vì không có hệ điều hành, chúng ta chỉ có thể lặp vô tận.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use core::arch::global_asm;

global_asm!(
    ".section .text._start",
    ".globl _start",
    "_start:",
    // Read hart ID into t0
    "csrr t0, mhartid",
    // Base address, only hart 0 continues, others wait
    "bnez t0, 1f",
    
    // Setup stack for hart 0. The stack grows downwards.
    "la sp, _stack_end",

    // Jump to the rust_main function
    "call rust_main",

    "1:",
    "wfi",
    "j 1b",
);

/// Điểm nhập của hệ điều hành.
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Sau này code khởi tạo UART và Kernel sẽ nằm ở đây
    let mut uart = Uart::new(0x10000000);
    uart.init();

    let _ = write!(uart, "Hello, xv6-rust!\n");

    let kernel_end = (&raw const _end) as usize;

    // Khởi tạo bộ cấp phát trang với vùng nhớ sau Kernel
    // Giả sử RAM kết thúc ở 0x81000000 để bắt đầu (có thể tăng lên sau)
    let mut curr = (kernel_end + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    while curr + PAGE_SIZE <= 0x8800_0000 {
        unsafe {
            ALLOCATOR.free(curr);
        }
        curr += PAGE_SIZE;
    }

    unsafe {
        // 1. Tạo bảng trang cho Kernel
        let kpt = vm::kvmmake(&mut ALLOCATOR, kernel_end);

        let _ = writeln!(uart, "Enabling paging...");

        // 2. Kích hoạt MMU
        vm::kvminit(kpt.entries);
    }

    let _ = writeln!(uart, "Virtual Memory Enabled!");

    loop {
        // Sử dụng inline assembly để tạm dừng CPU tiết kiệm điện
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

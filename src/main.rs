#![no_std]
#![no_main]

use core::arch::{asm, naked_asm};
use core::panic::PanicInfo;

// Entry point: naked function, không có prologue/epilogue
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        // Chỉ hart 0 được chạy, các hart khác ngủ
        "csrr t0, mhartid",
        "bnez t0, 1f",

        // Thiết lập stack pointer
        "la   sp, _stack_top",

        // Xóa BSS
        "la   t0, _bss_start",
        "la   t1, _bss_end",
        "2:",
        "bgeu t0, t1, 3f",
        "sd   zero, 0(t0)",
        "addi t0, t0, 8",
        "j    2b",
        "3:",

        // Nhảy vào kernel main
        "call kmain",

        // Nếu kmain return (không nên) thì loop mãi
        "1:",
        "wfi",
        "j 1b",

    )
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

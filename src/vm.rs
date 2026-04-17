use crate::kalloc::{PageAllocator, PAGE_SIZE};
use core::ptr::{read_volatile, write_volatile};

pub type PhysAddr = usize;
pub type VirtAddr = usize;

// Các bit cờ trong PTE (Page Table Entry)
pub const PTE_V: usize = 1 << 0;
pub const PTE_R: usize = 1 << 1;
pub const PTE_W: usize = 1 << 2;
pub const PTE_X: usize = 1 << 3;
pub const PTE_U: usize = 1 << 4;

/// Đại diện cho một bảng trang. Mỗi bảng có 512 mục (8 bytes mỗi mục = 4096 bytes)
pub struct PageTable {
    pub entries: *mut usize,
}

impl PageTable {
    /// Ánh xạ một dải địa chỉ ảo sang địa chỉ vật lý
    /// # Safety:
    /// Phải đảm bảo allocator đã được khởi tạo và vùng nhớ chưa bị ánh xạ trùng lặp.
    pub unsafe fn map(
        &mut self,
        va: VirtAddr,
        pa: PhysAddr,
        size: usize,
        perm: usize,
        allocator: &mut PageAllocator,
    ) {
        let mut start = va & !(PAGE_SIZE - 1);
        let last = (va + size - 1) & !(PAGE_SIZE - 1);
        let mut curr_pa = pa;

        while start <= last {
            let pte = self.walk(start, true, allocator);
            // Ghi địa chỉ vật lý và cờ vào PTE
            // Công thức RISC-V: PA >> 12 đưa vào bits 10-53 của PTE
            let val = ((curr_pa >> 12) << 10) | perm | PTE_V;
            write_volatile(pte, val);

            start += PAGE_SIZE;
            curr_pa += PAGE_SIZE;
        }
    }

    /// Tìm kiếm (hoặc tạo mới) PTE cho một địa chỉ ảo
    unsafe fn walk(
        &mut self,
        va: VirtAddr,
        alloc: bool,
        allocator: &mut PageAllocator,
    ) -> *mut usize {
        let mut table_ptr = self.entries;

        // Đi qua 3 tầng (L2, L1) để tìm tầng L0
        for level in (1..3).rev() {
            let idx = (va >> (12 + level * 9)) & 0x1FF;
            let pte = table_ptr.add(idx);

            if (read_volatile(pte) & PTE_V) != 0 {
                // Tầng tiếp theo đã tồn tại
                table_ptr = ((read_volatile(pte) >> 10) << 12) as *mut usize;
            } else {
                if !alloc {
                    return core::ptr::null_mut();
                }

                // Cấp phát trang mới cho bảng trang tầng tiếp theo
                let new_page = allocator
                    .alloc()
                    .expect("VM: Out of memory for page tables");
                // Xóa sạch trang mới (ghi 0)
                core::ptr::write_bytes(new_page, 0, PAGE_SIZE);

                write_volatile(pte, ((new_page as usize >> 12) << 10) | PTE_V);
                table_ptr = new_page as *mut usize;
            }
        }
        // Trả về địa chỉ của PTE ở tầng cuối cùng (L0)
        let idx = (va >> 12) & 0x1FF;
        table_ptr.add(idx)
    }
}

// Thêm vào src/vm.rs

/// Khởi tạo bảng trang cho Kernel (Kernel Page Table)
pub unsafe fn kvmmake(allocator: &mut PageAllocator, kernel_end: usize) -> PageTable {
    let mut kpt = PageTable {
        entries: allocator.alloc().expect("VM: Cannot alloc root table") as *mut usize,
    };
    // Xóa sạch bảng trang gốc
    core::ptr::write_bytes(kpt.entries as *mut u8, 0, PAGE_SIZE);

    // 1. Ánh xạ UART (Identity mapping)
    // Giúp chúng ta tiếp tục in được log sau khi bật MMU
    kpt.map(
        0x1000_0000,
        0x1000_0000,
        PAGE_SIZE,
        PTE_R | PTE_W,
        allocator,
    );

    // 2. Ánh xạ Kernel Code (Identity mapping)
    // Từ 0x80000000 đến điểm kết thúc của kernel trong RAM
    let kernel_size = kernel_end - 0x8000_0000;
    kpt.map(
        0x8000_0000,
        0x8000_0000,
        kernel_size,
        PTE_R | PTE_W | PTE_X,
        allocator,
    );

    kpt
}

/// Ghi bảng trang vào thanh ghi satp và bật MMU
pub unsafe fn kvminit(root_table_ptr: *mut usize) {
    // Sv39 mode là 8 (nằm ở bit 60-63 trên RISC-V 64)
    let satp_val = (8usize << 60) | ((root_table_ptr as usize) >> 12);

    // Sử dụng Inline Assembly để ghi vào thanh ghi điều khiển của CPU
    core::arch::asm!(
        "csrw satp, {0}",       // Ghi giá trị vào thanh ghi satp
        "sfence.vma zero, zero", // Xóa cache TLB để đảm bảo các ánh xạ mới có hiệu lực
        in(reg) satp_val
    );
}

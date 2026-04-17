use core::ptr::NonNull;

// Kích thước một trang là 4096 bytes
pub const PAGE_SIZE: usize = 4096;

/// Một mắt xích trong danh sách các trang trống.
/// Cấu trúc này sẽ được ghi đè trực tiếp lên đầu của mỗi trang RAM trống.
struct FreePage {
    next: Option<NonNull<FreePage>>,
}

pub struct PageAllocator {
    head: Option<NonNull<FreePage>>,
}

impl PageAllocator {
    pub const fn new() -> Self {
        Self { head: None }
    }

    /// Đưa một trang vào danh sách quản lý
    /// # Safety:
    /// Người gọi phải đảm bảo start đến start + PAGE_SIZE là vùng nhớ hợp lệ
    /// và không bị sử dụng bởi bất kỳ ai khác.
    pub unsafe fn free(&mut self, start: usize) {
        // Ép kiểu địa chỉ thành con trỏ tới FreePage
        let ptr = start as *mut FreePage;

        // Tạo một node mới trỏ tới head hiện tại
        let mut new_node = FreePage { next: self.head };

        unsafe {
            // Ghi node này vào đầu trang nhớ
            ptr.write(new_node);

            // Cập nhật head của allocator
            self.head = Some(NonNull::new_unchecked(ptr));
        }
    }

    /// Lấy ra một trang trống
    pub fn alloc(&mut self) -> Option<*mut u8> {
        self.head.map(|node_ptr| unsafe {
            let node = node_ptr.as_ref();
            self.head = node.next;
            node_ptr.as_ptr() as *mut u8
        })
    }
}

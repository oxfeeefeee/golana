//!
//!
//!

use solana_program::entrypoint::HEAP_START_ADDRESS;
use std::{alloc::Layout, mem::size_of, ptr::null_mut, usize};

const HEAP_LENGTH_MAX: usize = 256 * 1024;
const REGION0_SIZE: usize = 16 * 1024;

pub struct DualMalloc {
    smalloc: smalloc::Smalloc<
        { HEAP_START_ADDRESS as usize + REGION0_SIZE },
        { HEAP_LENGTH_MAX - REGION0_SIZE },
        64,
        1024,
    >,
}

impl DualMalloc {
    #[cfg(target_os = "solana")]
    pub const fn new() -> DualMalloc {
        DualMalloc {
            smalloc: smalloc::Smalloc::new(),
        }
    }

    unsafe fn use_smalloc() -> bool {
        const USE_SMALLOC: *const bool = HEAP_START_ADDRESS as usize as *const bool;
        *USE_SMALLOC
    }

    pub fn set_use_smalloc(use_smalloc: bool) {
        unsafe {
            const USE_SMALLOC: *mut bool = HEAP_START_ADDRESS as usize as *mut bool;
            *USE_SMALLOC = use_smalloc;
        }
    }

    fn allocated_by_smalloc(ptr: *mut u8) -> bool {
        ptr as usize >= HEAP_START_ADDRESS as usize + REGION0_SIZE
    }

    pub const fn smalloc_size() -> usize {
        HEAP_LENGTH_MAX - REGION0_SIZE
    }

    pub fn smalloc_mem_as_slice() -> &'static mut [u8] {
        unsafe {
            let start = HEAP_START_ADDRESS as usize + REGION0_SIZE;
            std::slice::from_raw_parts_mut(start as *mut u8, Self::smalloc_size())
        }
    }
}

unsafe impl std::alloc::GlobalAlloc for DualMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if Self::use_smalloc() {
            self.smalloc.alloc(layout)
        } else {
            const START: usize =
                solana_program::entrypoint::HEAP_START_ADDRESS as usize + size_of::<bool>();
            const POS_PTR: *mut usize = START as *mut usize;
            const TOP_ADDRESS: usize = HEAP_START_ADDRESS as usize + REGION0_SIZE;
            const BOTTOM_ADDRESS: usize = START + size_of::<*mut u8>();

            let mut pos = *POS_PTR;
            if pos == 0 {
                // First time, set starting position
                pos = TOP_ADDRESS;
            }
            pos = pos.saturating_sub(layout.size());
            pos &= !(layout.align().saturating_sub(1));
            if pos < BOTTOM_ADDRESS {
                return null_mut();
            }
            *POS_PTR = pos;
            pos as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // region0 is a bump allocator, so we don't need to do anything in that case
        if Self::allocated_by_smalloc(ptr) {
            self.smalloc.dealloc(ptr, layout);
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if Self::use_smalloc() && Self::allocated_by_smalloc(ptr) {
            // dealloc and alloc by smalloc, most cases
            self.smalloc.realloc(ptr, layout, new_size)
        } else {
            let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
            let new_ptr = self.alloc(new_layout);
            if !new_ptr.is_null() {
                std::ptr::copy_nonoverlapping(ptr, new_ptr, std::cmp::min(layout.size(), new_size));
                self.dealloc(ptr, layout);
            }
            new_ptr
        }
    }
}

//! bump allocator for kernel init. should not be used at all afterwards

use crate::mm::paging::{PageDirectory, PageManager};
use alloc::alloc::Layout;
use log::{debug, trace};

const BUMP_ALLOC_SIZE: usize = 0x40000; // 256k

static mut ALLOC_ADDR_INITIAL: usize = 0; // initial alloc addr
static mut ALLOC_ADDR: usize = 0; // to be filled in with end of kernel on init
static mut ALLOC_AREA: [u8; BUMP_ALLOC_SIZE] = [0; BUMP_ALLOC_SIZE]; // hopefully this will just be located in bss? we can't just allocate memory for it since we need it to allocate memory
static mut ALLOC_OFFSET: usize = 0;

/// result of bump_alloc calls
pub struct AllocResult<T> {
    pub pointer: *mut T,
    pub phys_addr: usize,
}

/// simple bump allocator, used to allocate memory required for initializing things
pub unsafe fn bump_alloc<T>(layout: Layout) -> AllocResult<T> {
    // check if alignment is requested and we aren't already aligned
    let align_inv = !(layout.align() - 1); // alignment is guaranteed to be a power of two
    if layout.align() > 1 && ALLOC_ADDR & align_inv > 0 {
        ALLOC_ADDR &= align_inv;
        ALLOC_ADDR += layout.align();
    }

    // increment address to make room for area of provided size, return pointer to start of area
    let tmp = ALLOC_ADDR;
    ALLOC_ADDR += layout.size();

    if ALLOC_ADDR >= ALLOC_ADDR_INITIAL + BUMP_ALLOC_SIZE {
        // prolly won't happen but might as well
        panic!("out of memory (bump_alloc)");
    }

    trace!("bump allocated virt {:#x}, phys {:#x}, size {:#x}", tmp + ALLOC_OFFSET, tmp, layout.size());

    AllocResult {
        pointer: (tmp + ALLOC_OFFSET) as *mut T,
        phys_addr: tmp,
    }
}

/// initialize the bump allocator
///
/// # Safety
///
/// this function is unsafe because if it's called more than once, the bump allocator will reset and potentially critical data can be overwritten
pub unsafe fn init_bump_alloc(offset: usize) {
    // calculate alloc addr for initial bump_alloc calls
    ALLOC_OFFSET = offset;
    ALLOC_ADDR_INITIAL = (&ALLOC_AREA as *const _) as usize - ALLOC_OFFSET;
    ALLOC_ADDR = ALLOC_ADDR_INITIAL;

    debug!("bump alloc @ {:#x} - {:#x} (virt @ {:#x})", ALLOC_ADDR, ALLOC_ADDR + BUMP_ALLOC_SIZE, ALLOC_ADDR + ALLOC_OFFSET);
}

/// frees unused memory from the bump allocator
///
/// # Safety
///
/// this function is unsafe because it accesses global mutable state without locking (tho the bump allocator really shouldn't be used before interrupts or bringup of other CPUs)
pub unsafe fn free_unused_bump_alloc<D: PageDirectory>(manager: &mut PageManager<D>, dir: &mut D) {
    let page_size = dir.page_size();
    let start = ((ALLOC_ADDR + ALLOC_OFFSET + page_size - 1) / page_size) * page_size;
    let end = ((ALLOC_ADDR_INITIAL + BUMP_ALLOC_SIZE + ALLOC_OFFSET) / page_size) * page_size;

    debug!("freeing unused {:#x} - {:#x}", start, end);

    for i in (start..end).step_by(page_size) {
        manager.free_frame(dir, i).unwrap();
    }
}
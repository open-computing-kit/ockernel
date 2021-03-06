pub mod ints;
pub mod gdt;
pub mod paging;
pub mod syscalls;
pub mod tasks;

use core::arch::asm;
use crate::platform::bootloader;

// various useful constants
pub const MEM_TOP: usize = 0xffffffff;
pub const LINKED_BASE: usize = 0xc0000000;
pub const KHEAP_START: usize = LINKED_BASE + 0x10000000;

pub const PAGE_SIZE: usize = 0x1000;
pub const INV_PAGE_SIZE: usize = !(PAGE_SIZE - 1);

pub const MAX_STACK_FRAMES: usize = 1024;

pub static mut MEM_SIZE: u64 = 0; // filled in later by BIOS or something similar

/// halt system
pub fn halt() -> ! {
    log!("halting");

    unsafe {
        loop {
            asm!("cli; hlt"); // clear interrupts, halt
        }
    }
}

/// initialize sub-modules
pub fn init() {
    debug!("bootloader pre init");
    unsafe { bootloader::pre_init(); }

    debug!("initializing GDT");
    unsafe { gdt::init(); }
    debug!("initializing interrupts");
    unsafe { ints::init(); }

    debug!("bootloader init");
    unsafe { bootloader::init(); }

    debug!("initializing paging");
    unsafe { paging::init(); }
}

pub fn init_after_heap() {
    debug!("bootloader init after heap");
    bootloader::init_after_heap();
}

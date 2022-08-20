#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(core_c_str)]
#![feature(cstr_from_bytes_until_nul)]

extern crate alloc;

pub mod logging;

use alloc::alloc::Layout;
use common::{
    arch::paging::PageDir,
    mm::{heap::CustomAlloc, paging::PageDirectory},
    BootModule, MemoryRegion,
};
use log::{debug, error, info, trace, warn};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[global_allocator]
static ALLOCATOR: CustomAlloc = CustomAlloc;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error with layout {:?}", layout);
}

#[panic_handler]
pub fn panic_implementation(info: &core::panic::PanicInfo) -> ! {
    let (file, line) = match info.location() {
        Some(loc) => (loc.file(), loc.line()),
        None => ("", 0),
    };

    if let Some(m) = info.message() {
        error!("PANIC: file='{}', line={} :: {}", file, line, m);
    } else if let Some(m) = info.payload().downcast_ref::<&str>() {
        error!("PANIC: file='{}', line={} :: {}", file, line, m);
    } else {
        error!("PANIC: file='{}', line={} :: ?", file, line);
    }

    unsafe {
        common::arch::halt();
    }
}

#[no_mangle]
pub extern "cdecl" fn _start(dir: PageDir, modules_ptr: *const BootModule, num_modules: u32, regions_ptr: *const MemoryRegion, num_regions: u32) -> ! {
    // initialize our logger
    logging::init().unwrap();

    info!("{} v{}", NAME, VERSION);
    //info!("Hellorld!");

    debug!("modules_ptr: {:?}, num_modules: {:?}", modules_ptr, num_modules);

    let modules = unsafe { core::slice::from_raw_parts(modules_ptr, num_modules as usize) };

    info!("{:?}", modules);

    debug!("regions_ptr: {:?}, num_regions: {:?}", regions_ptr, num_regions);

    let regions = unsafe { core::slice::from_raw_parts(regions_ptr, num_regions as usize) };

    info!("{:?}", regions);

    unsafe {
        common::arch::halt();
    }
}
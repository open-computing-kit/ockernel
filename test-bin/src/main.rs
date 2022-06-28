#![no_std]
#![no_main]

#[path="../..//syscalls.rs"]
pub mod syscalls;

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    syscalls::test_log(b"panic :(\0");
    syscalls::exit();
}

static mut TEST_STATIC: usize = 0;

#[no_mangle]
fn _start() {
    if syscalls::is_computer_on() {
        syscalls::test_log(b"computer is on\0");
    } else {
        syscalls::test_log(b"computer is not on\0");
    }

    unsafe { TEST_STATIC = 621; }

    if unsafe { TEST_STATIC == 621 } {
        syscalls::test_log(b"TEST_STATIC is set\0");
    } else {
        syscalls::test_log(b"TEST_STATIC is not set\0");
    }

    if syscalls::fork() != 0 {
        syscalls::test_log(b"parent\0");

        if unsafe { TEST_STATIC == 621 } {
            syscalls::test_log(b"parent: preserved\0");
        }

        syscalls::exec(b"/fs/initrd/test-bin2\0");
    } else {
        syscalls::test_log(b"child\0");

        if unsafe { TEST_STATIC == 621 } {
            syscalls::test_log(b"child: preserved\0");
        }
    }

    let proc = syscalls::fork();

    if proc != 0 {
        for _i in 0..8 {
            for _i in 0..1024 * 1024 { // slow things down
                unsafe {
                    asm!("nop");
                }
            }

            syscalls::test_log(b"OwO\0");

            for _i in 0..1024 * 1024 {
                unsafe {
                    asm!("nop");
                }
            }
        }

        unsafe {
            asm!("int3"); // effectively crash this process
        }

        loop {}
    } else {
        for _i in 0..32 {
            syscalls::test_log(b"UwU\0");

            for _i in 0..1024 * 1024 * 2 { // slow things down
                unsafe {
                    asm!("nop");
                }
            }
        }
    }

    panic!("OwO");
}
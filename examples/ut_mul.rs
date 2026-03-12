#![no_std]
#![no_main]

use core::arch::asm;

fn exit(code: i8) -> ! {
    unsafe {
        asm!("mv a0, {0}",
             "li a7, 93",
             "ecall",
             in(reg) code,
        )
    }
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    exit(-128);
}

#[unsafe(no_mangle)]
fn abort() -> ! {
    panic!("abort!")
}

#[unsafe(no_mangle)]
fn _start() -> ! {
    ckb_alt_bn128::ethereum::ut::test_alt_bn128_mul();
    exit(0)
}

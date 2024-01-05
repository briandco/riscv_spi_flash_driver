#![no_std]
#![no_main]
#![feature(asm)]

// use cortex_m_rt::entry;
use riscv_rt::entry;
pub mod vajra;
// pub mod start;
// pub mod trap;

// #[cfg(feature = "riscv64")]
const REGSIZE: usize = 8;

// #[cfg(not(feature = "riscv64"))]
// const REGSIZE: usize = 4;

// #[cfg(feature = "riscv64")]
const LREG: &str = "ld";

// #[cfg(not(feature = "riscv64"))]
// const LREG: &str = "lw";

// #[cfg(feature = "riscv64")]
const SREG: &str = "sd";

// #[cfg(not(feature = "riscv64"))]
// const SREG: &str = "sw";

const MSTATUS_MPP: u32 = 0x00001800;
const MSTATUS_FS: u32 = 0x00006000;

#[entry]
fn main() -> !{
    let x = add_variable(5, 10);
    loop{}
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { riscv::asm::nop() };
    }
}

fn add_variable(a: i32, b: i32) -> i32 {
    a + b
}
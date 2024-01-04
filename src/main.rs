#![no_std]
#![no_main]
#![feature(asm)]

// use cortex_m_rt::entry;
// use riscv_rt::entry;
pub mod vajra;
pub mod start;
pub mod trap;

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

// #[entry]
#[no_mangle]
pub unsafe fn start_shakti() -> !{
    // let object = vajra::FlashWriterEraser::new();
    vajra::uart_init();
    vajra::write_uart_string("Hello world from rust".as_ptr());
    loop{}
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { riscv::asm::nop() };
    }
}

#![no_std]
#![no_main]
#![feature(asm)]

// use cortex_m_rt::entry;
use riscv_rt::entry;
pub mod vajra;
pub mod start;
pub mod trap;

#[entry]
// #[no_mangle]
fn main() -> !{
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

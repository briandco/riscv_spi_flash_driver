#![no_std]
#![no_main]
#![feature(asm)]

// use cortex_m_rt::entry;
use riscv_rt::entry;
use uart::{UartInner, UART_OFFSET};
pub mod uart;
pub mod common;
pub mod spi;

#[entry]
fn main() -> ! {

    let mut uart = unsafe{UartInner::new(UART_OFFSET)};
    uart.write_uart_char('B');
    
    let x = add_variable(5, 10);
    loop {}
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

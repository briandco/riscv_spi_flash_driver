#![no_std]
#![no_main]
#![feature(asm)]

// use cortex_m_rt::entry;
use riscv_rt::entry;
pub mod uart;

#[entry]
fn main() -> ! {
    let char_a = uart::test_tock_reg();
    uart::write_uart_char();
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

//uart_init();
// vajra::uart_init();
//let mut instance = unsafe { UART_INSTANCE }[0].unwrap();
//vajra::write_uart_string("Hello world from rust".as_ptr());

//vajra::write_uart_character(&mut instance, b'x');
// write_uart_character('c');

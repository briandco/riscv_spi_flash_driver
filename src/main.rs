#![no_std]
#![no_main]
#![feature(asm)]

// use cortex_m_rt::entry;
use riscv_rt::entry;
use uart::PanicUart;
pub mod uart;

#[entry]
fn main() -> ! {
    //let x = add_variable(5, 10);
    unsafe {
        let mut u0 = uart::UartInner::new(0x00011300);
        u0.init();
        let x = add_variable(5, 10);
        u0.write_char('c');
    };

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

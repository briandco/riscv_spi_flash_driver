#![no_std]
#![no_main]

// use cortex_m_rt::entry;
use riscv_rt::entry;
pub mod vajra;
// pub mod spi;

#[entry]
fn main() -> !{
    // let object = vajra::FlashWriterEraser::new();
    let uart = vajra::Uart::new(0x00011300);
    let mut uart_object = uart.init_uart();
    vajra::write_uart_string(&mut uart_object, "Hello world from rust".as_ptr());
    loop{}
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { riscv::asm::nop() };
    }
}

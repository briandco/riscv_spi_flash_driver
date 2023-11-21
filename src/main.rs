#![no_std]
#![no_main]

use cortex_m_rt::entry;
pub mod vajra;
pub mod spi;

#[entry]
fn main() -> !{
    let object = vajra::FlashWriterEraser::new();
    object.
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

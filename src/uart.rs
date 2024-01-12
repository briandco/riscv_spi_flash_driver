use core::{marker::PhantomData, ops};

use riscv::asm::nop;
use tock_registers::{
    interfaces::{Readable, Writeable, ReadWriteable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly, self},
};

use self::{UCR::PARITY::Unused, UBR::BAUD};

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

// PL011 UART registers.
//
// Descriptions taken from "PrimeCell UART (PL011) Technical Reference Manual" r1p5.

pub const BREAK_ERROR: u8 = 1 << 7;
pub const FRAME_ERROR: u8 = 1 << 6;
pub const OVERRUN: u8 = 1 << 5;
pub const PARITY_ERROR: u8 = 1 << 4;
pub const STS_RX_FULL: u8 = 1 << 3;
pub const STS_RX_NOT_EMPTY: u8 = 1 << 2;
pub const STS_TX_FULL: u8 = 1 << 1;
pub const STS_TX_EMPTY: u8 = 1 << 0;

register_structs! {
    #[allow(non_snake_case)]
    pub Registers{
        (0x00 => UBR: ReadWrite<u32>),
       // (0x02 => _reserved0),
        (0x04 => TX_REG: WriteOnly<u32>),
        (0x08 => RCV_REG: ReadOnly<u32, RCV_REG::Register>),
        (0x0C => USR : ReadOnly<u32, USR::Register>),
        //(0x0D => _reserved1),
        //(0x0E => _reserved2),
        (0x10 => DELAY: ReadWrite<u32, DELAY::Register>),
        //(0x12 => _reserved3),
        (0x14 => UCR: ReadWrite<u32, UCR::Register>),
        //(0x16 => _reserved4),
        (0x18 => IEN: ReadWrite<u32, IEN::Register>),
        //(0x1A => _reserved5),
        (0x1C => IQCYCLES: ReadWrite<u32, IQCYCLES::Register>),
        //(0x1D => _reserved6),
        //(0x1E => _reserved7),
        (0x20 => RX_THRESHOLD: WriteOnly<u32, RX_THRESHOLD::Register>),
        //(0x21 => _reserved8),
        //(0x22 => _reserved9),
        (0x24 => @END),
    }
}

register_bitfields! {
    u32,

    UBR [
        BAUD OFFSET(0) NUMBITS(16) []
    ],

    /// UART Status register
    USR [
        /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// Line Control Register, LCR_H.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is empty.
        /// - If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
        /// - This bit does not indicate if there is data in the transmit shift register.
        BREAK_ERROR OFFSET(7) NUMBITS(1) [],
        //Break Error (Sets when the data and stop are both zero
        FRAME_ERROR OFFSET(6) NUMBITS(1) [],
        //Frame Error (Sets when the stopis zero)
        OVERRUN OFFSET(5) NUMBITS(1) [],
        //Overrun Error (A data overrun error occurred in the receive
        //shift register. This happens when additional data arrives
        //while the FIFO is full. )
        PARITY_ERROR OFFSET(4) NUMBITS(1) [],
        //Parity Error (Sets when The receive character does not
        //have correct parity information and is suspect.

        STS_RX_FULL OFFSET(3) NUMBITS(1) [],
        //Receiver Full (Sets when the Receive Buffer is Full)

        STS_RX_NOT_FULL OFFSET(2) NUMBITS(1) [],
        // Receiver Not Empty (Sets when there is some data in the
        //Receive Buffer).
        STS_TX_FULL OFFSET(1) NUMBITS(1) [],
        //Transmitter Full (Sets when the transmit Buffer is full)
        STS_TX_EMPTY OFFSET(0) NUMBITS(1) []
        //Transmitter Empty(Sets when the Transmit Buffer is empty).
    ],

    UCR [
        UART_TX_RX_LEN OFFSET(5) NUMBITS(6) [],
        //Character size of data. Maximum length is 32 bits.
        PARITY OFFSET(3) NUMBITS(2) [
            None = 0b00,
            Odd = 0b01,
            Even = 0b10,
            Unused = 0b11

        ],
         //Insert Parity bits
         //00 - None
         //01 - Odd
        //10- Even
        // 11 - Unused or Undefined
        STOP_BITS OFFSET(1) NUMBITS(2) [

            StopBits1 = 0b00,
            StopBits1_5 = 0b01,
            StopBits2 = 0b10


        ],
        //Stop bits
       //00 - 1 Stop bits
        //01 - 1.5 Stop bits
        //10 - 2 Stop bits
    ],

    TX_REG [

        TX_DATA OFFSET(0) NUMBITS(32) []
    ],


    RCV_REG [

        RX_DATA OFFSET(0) NUMBITS(32) []
    ],

    IEN [

        ENABLE_TX_EMPTY OFFSET(0) NUMBITS(1) [],
        ENABLE_TX_FULL OFFSET(1) NUMBITS(1) [],
        ENABLE_RX_NOT_EMPTY OFFSET(2) NUMBITS(1) [],
        ENABLE_RX_FULL OFFSET(3) NUMBITS(1) [],
        ENABLE_PARITY_ERROR OFFSET(4) NUMBITS(1) [],
        ENABLE_OVERRUN OFFSET(5) NUMBITS(1) [],
        ENABLE_FRAME_ERROR OFFSET(6) NUMBITS(1) [],
        ENABLE_BREAK_ERROR OFFSET(7) NUMBITS(1) [],
        ENABLE_RX_THRESHOLD OFFSET(8) NUMBITS(1) []
    ],

      DELAY [
        COUNT OFFSET(0) NUMBITS(8) []
      ],

      IQCYCLES[
        COUNT OFFSET(0) NUMBITS(8) []
      ],
    RX_THRESHOLD [
        ///  The fractional baud rate divisor.
        FIFO_RX OFFSET(0) NUMBITS(8) []
    ]

}

pub fn test_tock_reg() -> char {
    let registers: *const Registers = 0x0001_1300 as *const Registers;
    unsafe {
        let baud_value = (*registers).UBR.get() as u8 as char;
        baud_value
    }
}

// raw access ==================================================
pub fn write_uart_char(){
    let registers: *const Registers = 0x0001_1300 as *const Registers;
    let c = 'A';
    unsafe{
        while (*registers).USR.matches_all(USR::STS_TX_FULL::SET){ 
            nop() ;
        }
        (*registers).TX_REG.set(c as u32);
    }
}


// pub struct MMIODerefWrapper<T> {
//     start_addr: usize,
//     phantom: PhantomData<fn() -> T>,
// }

// impl<T> MMIODerefWrapper<T> {
//     /// Create an instance.
//     pub const unsafe fn new(start_addr: usize) -> Self {
//         Self {
//             start_addr,
//             phantom: PhantomData,
//         }
//     }
// }

// impl<T> ops::Deref for MMIODerefWrapper<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         unsafe { &*(self.start_addr as *const T) }
//     }
// }

// type Registers = MMIODerefWrapper<RegisterBlock>;

// #[derive(PartialEq)]
// enum BlockingMode {
//     Blocking,
//     NonBlocking,
// }

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

// pub struct UartInner {
//     registers: Registers,
//     chars_written: usize,
//     chars_read: usize,
// }

// // Export the inner struct so that BSPs can use it for the panic handler.
// pub use UartInner as PanicUart;

// /// Representation of the UART.
// // pub struct Uart {
// //     inner: UartInner,
// // }

// //--------------------------------------------------------------------------------------------------
// // Public Code
// //--------------------------------------------------------------------------------------------------

// impl UartInner {
//     /// Create an instance.
//     ///
//     /// # Safety
//     ///
//     /// - The user must ensure to provide a correct MMIO start address.
//     pub const unsafe fn new(start_addr: usize) -> Self {
//         Self {
//             registers: Registers::new(start_addr),
//             chars_written: 0,
//             chars_read: 0,
//         }
//     }

//     pub fn init(&mut self) {
//         // Turn the UART off temporarily.
//         self.registers.UCR.set(0);

//         // Clear all pending interrupts.
//         self.registers.IEN.set(0x0000);

//         /// Clock = 50Mhz
//         /// baud value = clock /(16 * baud rate)
//         /// baud value  = 50 Mhz /(16 * 19200)  = 163 => 0xA3
//         self.registers.UBR.set(0xA3);
        
//     }

//     /// Send a character.
//     pub fn write_char(&mut self, c: char) {
//         // Spin while TX FIFO full is set, waiting for an empty slot.
//         while self.registers.USR.matches_all(USR::STS_TX_FULL::SET)
//         //.matches_all(USR::STS_TX_FULL::SET) {
//         { //cpu_core::nop();
//             unsafe { nop() };
//         }

//         // Write the character to the buffer.
//         self.registers.TX_REG.set(c as u32);

//         //self.chars_written += 1;
//     }
// }

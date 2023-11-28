//! NVMC (i.e. flash) driver for the nrf52840 board, written in pure-rust.

// use crate::FlashInterface;
use nrf9160_constants::*;
// use crate::spi;
use core::ptr;

#[rustfmt::skip]
mod nrf9160_constants {
    pub const FLASH_PAGE_SIZE : u32 = 4096;
    pub const STACK_LOW       : u32 = 0x20_000_000;
    pub const STACK_UP        : u32 = 0x20_040_000;
    pub const RB_HDR_SIZE     : u32 = 0x100;
    pub const BASE_ADDR       : u32 = 0x2f000;
    pub const VTR_TABLE_SIZE  : u32 = 0x100;
    pub const FW_RESET_VTR    : u32 = BASE_ADDR + RB_HDR_SIZE + VTR_TABLE_SIZE + 1;
}

#[derive(Copy, Clone)]
/// Struct representing a NS16550A UART peripheral
pub struct Uart {
    /// Base address of the peripheral
    base_address: usize,
}

#[cfg(feature = "use_rx_threshold")]
pub struct UartStruct {
    baud: u16,
    reserv0: u16,
    tx_reg: u32,
    rcv_reg: u32,
    status: u8,
    reserv1: u8,
    reserv2: u16,
    delay: u16,
    reserv3: u16,
    control: u16,
    reserv5: u16,
    ien: u8,
    reserv6: u8,
    reserv7: u16,
    iqcycles: u8,
    reserv8: u8,
    reserv9: u16,
    rx_threshold: u8,
    reserv10: u8,
    reserv11: u16,
}

#[cfg(not(feature = "use_rx_threshold"))]
#[derive(Copy, Clone)]
pub struct UartStruct {
    baud: u16,
    reserv0: u16,
    tx_reg: u32,
    rcv_reg: u32,
    status: u8,
    reserv1: u8,
    reserv2: u16,
    delay: u16,
    reserv3: u16,
    control: u16,
    reserv5: u16,
    ien: u8,
    reserv6: u8,
    reserv7: u16,
    iqcycles: u8,
    reserv8: u8,
    reserv9: u16,
}

// Define constants for bit masks
// pub const STS_RX_THRESHOLD: u8 = 1 << 8;
pub const BREAK_ERROR: u8 = 1 << 7;
pub const FRAME_ERROR: u8 = 1 << 6;
pub const OVERRUN: u8 = 1 << 5;
pub const PARITY_ERROR: u8 = 1 << 4;
pub const STS_RX_FULL: u8 = 1 << 3;
pub const STS_RX_NOT_EMPTY: u8 = 1 << 2;
pub const STS_TX_FULL: u8 = 1 << 1;
pub const STS_TX_EMPTY: u8 = 1 << 0;

const MAX_UART_COUNT: usize = 3/* your value here */;
const UART_OFFSET: usize = 0x100/* your value here */;
const UART0_START: usize = 0x00011300/* your value here */;

// Assuming you have already defined the UartStruct as mentioned earlier

const ARRAY_REPEAT_VALUE: Option<&'static mut UartStruct> = None;

pub static mut UART_INSTANCE: [Option<&'static mut UartStruct>; MAX_UART_COUNT] = [ARRAY_REPEAT_VALUE; MAX_UART_COUNT];

// pub UART_INSTANCE:UartStruct = [0..MAX_UART_COUNT];

impl Uart {
    // pub fn new()-> Self {
    //     UartStruct { baud: 19200, reserv0: (), tx_reg: (), rcv_reg: (), status: (), reserv1: (), reserv2: (), delay: (), reserv3: (), control: (), reserv5: (), ien: (), reserv6: (), reserv7: (), iqcycles: (), reserv8: (), reserv9: () }
    // }
    pub fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    pub fn init_uart(&self) -> UartStruct {
        let raw_ptr  =  self.base_address as *mut UartStruct;
        let rust_reference: UartStruct = unsafe{ *raw_ptr.as_mut().unwrap() };
        rust_reference
    }
}

// pub unsafe fn uart_init() -> [Option<&'static mut UartStruct>; 3] {
//     for i in 0..MAX_UART_COUNT {
//         let uart_address = UART0_START + i * UART_OFFSET;
//         UART_INSTANCE[i] = Some(&mut *(uart_address as *mut UartStruct));
//     }
//     UART_INSTANCE
// }

pub fn write_uart_character(instance: &mut UartStruct, prn_character: u8) -> u32 {
    while instance.status & STS_TX_FULL != 0 {
        // Wait until the TX buffer is not full
    }

    // Write the character to the TX register
    instance.tx_reg = prn_character as u32;

    0 // Return value, change if necessary
}


pub fn write_uart_string(instance: &mut UartStruct, ptr_string: *const u8) -> u32 {
    let mut i = 0;
    let mut temp: u8;

    unsafe {
        // Access the string using unsafe Rust code since it's a raw pointer
        loop {
            temp = ptr::read_volatile(ptr_string.offset(i as isize));
            i += 1;

            // Break the loop when reaching the null terminator
            if temp == 0 {
                break;
            }

            // Write each character to the UART
            write_uart_character(instance, temp);
        }
    }

    0 // Return value, change if necessary
}

pub struct FlashWriterEraser {
    pub nvmc: u8,
}

// impl FlashWriterEraser {
//     pub fn new() -> Self {
//         FlashWriterEraser {
//             nvmc: 0,
//         }
//     }

//     fn hal_flash_write(&self, address: usize, data: *const u8, len: usize) {
    //     let address = address as u32;
    //     let len = len as u32;

    //     let mut idx = 0u32;
    //     let mut src = data as *mut u32;
    //     let mut dst = address as *mut u32;

    //     while idx < len {
    //         let data_ptr = (data as *const u32) as u32;
    //         // Check if the following holds true and do a full word write i.e. 4-byte write
    //         // - if `len - idx` is greater than 3 (i.e. 4 bytes).
    //         // - if the address is aligned on a word (i.e. 4-byte) boundary.
    //         // - if the data_ptr is aligned on a word (i.e. 4-byte) boundary.
    //         if ((len - idx > 3)
    //             && ((((address + idx) & 0x03) == 0) && ((data_ptr + idx) & 0x03) == 0))
    //         {
    //             // Enable NVM writes
    //             self.nvmc.config.write(|w| w.wen().wen());
    //             while self.nvmc.readynext.read().readynext().is_busy() {}
    //             unsafe {
    //                 *dst = *src; // 4-byte write
    //             };
    //             // Wait until writing is done
    //             while self.nvmc.ready.read().ready().is_busy() {}
    //             src = ((src as u32) + 4) as *mut u32; // increment pointer by 4
    //             dst = ((dst as u32) + 4) as *mut u32; // increment pointer by 4
    //             idx += 4;
    //         } else {
    //             // else do a single byte write i.e. 1-byte write
    //             let mut val = 0u32;
    //             let val_bytes = ((&mut val) as *mut u32) as *mut u8;
    //             let offset = (address + idx) - (((address + idx) >> 2) << 2); // offset from nearest word aligned address
    //             dst = ((dst as u32) - offset) as *mut u32; // subtract offset from dst addr
    //             unsafe {
    //                 val = *dst; // assign current val at dst to val
    //                             // store data byte at idx to `val`. `val_bytes` is a byte-pointer to val.
    //                 *val_bytes.add(offset as usize) = *data.add(idx as usize);
    //             }

    //             // Enable NVM writes
    //             self.nvmc.config.write(|w| w.wen().wen());
    //             while self.nvmc.readynext.read().readynext().is_busy() {}
    //             unsafe {
    //                 *dst = val; // Technically this is a 1-byte write ONLY
    //                             // but only full 32-bit words can be written to Flash using the NVMC interface
    //             };
    //             // Wait until writing is done
    //             while self.nvmc.ready.read().ready().is_busy() {}
    //             src = ((src as u32) + 1) as *mut u32; // increment pointer by 1
    //             dst = ((dst as u32) + 1) as *mut u32; // increment pointer by 1
    //             idx += 1;
    //         }
    //     }
    // }

    
    

    // Assuming you have already defined the UartStruct and status bit masks as mentioned earlier

    
    
    // fn hal_flash_erase(&self, addr: usize, len: usize) {
    //     // let starting_page = addr as u32;
    //     // let ending_page = (addr + len) as u32;
    //     configure_spi(SPI0_OFFSET);
    //     spi_init();

    //     printf("SPI init done\n");

    //     flash_device_id();

    //     waitfor(200);
    //     flash_cmd_addr(0xdc000000, addr);
    // }

    // fn hal_init() {}
    // fn hal_flash_lock(&self) {}
    // fn hal_flash_unlock(&self) {}
// }



use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

// PL011 UART registers.
//
// Descriptions taken from "PrimeCell UART (PL011) Technical Reference Manual" r1p5.
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
            StopBits1.5 = 0b01,
            StopBits2 = 0b10
           
        
        ],
        //Stop bits
       //00 - 1 Stop bits
        //01 - 1.5 Stop bits
        //10 - 2 Stop bits
    ],

    /// Integer Baud Rate Divisor.
    TX_REG [
        /// The integer baud rate divisor.
        TX_DATA OFFSET(0) NUMBITS(32) []
    ],

    /// Fractional Baud Rate Divisor.
    RCV_REG [
        ///  The fractional baud rate divisor.
        RX_DATA OFFSET(0) NUMBITS(32) []
    ],

    IEN [
        ///  The fractional baud rate divisor.
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
      ]
    
      IQCYCLES[
        COUNT OFFSET(0) NUMBITS(8) []
      ]
    RX_THRESHOLD [
        ///  The fractional baud rate divisor.
        FIFO_RX OFFSET(0) NUMBITS(8) []
    ]
  
}




register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => UBR: ReadWrite<u16>),
        (0x04 => TX_REG: WriteOnly<u32>),
        (0x08 => RCV_REG: ReadOnly<u32, RCV_REG::Register>),
        (0x0C => USR : ReadOnly<u32, USR::Register>),
        (0x10 => DELAY: ReadWrite<u16, DELAY::Register>),
        (0x14 => UCR: ReadWrite<u16, UCR::Register>),
        (0x18 => IEN: ReadWrite<u16, IEN::Register>),
        (0x1C => IQCYCLES: ReadWrite<u8, IQCYCLES::Register>),
        (0x20 => RX_THRESHOLD: WriteOnly<u8, RX_THRESHOLD::Register>),
        (0x24 => @END),
    }
}
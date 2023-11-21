use self::spi_registers::spi_cr1;

const SPI0_OFFSET: u32 = 0x00000000;
const SPI1_OFFSET: u32 = 0x00000100;
const SPI2_OFFSET: u32 = 0x00000200;

const SPI_CR1: u32 = 0x00020000;
const SPI_CR2: u32 = 0x00020004;
const SPI_SR: u32 = 0x00020008;
const SPI_DR1: u32 = 0x0002000C;
const SPI_DR2: u32 = 0x00020010;
const SPI_DR3: u32 = 0x00020014;
const SPI_DR4: u32 = 0x00020018;
const SPI_DR5: u32 = 0x0002001C;
const SPI_CRCPR: u32 = 0x00020020;
const SPI_RXCRCR: u32 = 0x00020024;
const SPI_TXCRCR: u32 = 0x00020028;

// defining SPI_CR1 register
const SPI_CPHA: u32 = 1 << 0;
const SPI_CPOL: u32 = 1 << 1;
const SPI_MSTR: u32 = 1 << 2;
const SPI_BR: fn(u32) -> u32 = |x| x << 3;
const SPI_SPE: u32 = 1 << 6;
const SPI_LSBFIRST: u32 = 1 << 7;
const SPI_SSI: u32 = 1 << 8;
const SPI_SSM: u32 = 1 << 9;
const SPI_RXONLY: u32 = 1 << 10;
const SPI_CRCL: u32 = 1 << 11;
const SPI_CCRCNEXT: u32 = 1 << 12;
const SPI_CRCEN: u32 = 1 << 13;
const SPI_BIDIOE: u32 = 1 << 14;
const SPI_BIDIMODE: u32 = 1 << 15;
const SPI_TOTAL_BITS_TX: fn(u32) -> u32 = |x| x << 16;
const SPI_TOTAL_BITS_RX: fn(u32) -> u32 = |x| x << 24;

// defining SPI_CR2 register
const SPI_RX_IMM_START: u32 = 1 << 16;
const SPI_RX_START: u32 = 1 << 15;
const SPI_LDMA_TX: u32 = 1 << 14;
const SPI_LDMA_RX: u32 = 1 << 13;
const SPI_FRXTH: u32 = 1 << 12;
const SPI_DS: fn(u32) -> u32 = |x| x << 8;
const SPI_TXEIE: u32 = 1 << 7;
const SPI_RXNEIE: u32 = 1 << 6;
const SPI_ERRIE: u32 = 1 << 5;
const SPI_FRF: u32 = 1 << 4;
const SPI_NSSP: u32 = 1 << 3;
const SPI_SSOE: u32 = 1 << 2;
const SPI_TXDMAEN: u32 = 1 << 1;
const SPI_RXDMAEN: u32 = 1 << 0;

// defining SR register
const SPI_FTLVL: fn(u32) -> u32 = |x| x << 11;
const SPI_FRLVL: fn(u32) -> u32 = |x| x << 9;
const SPI_FRE: u32 = 1 << 8;
const SPI_OVR: u32 = 1 << 6;
const SPI_MODF: u32 = 1 << 5;
const SPI_CRCERR: u32 = 1 << 4;
const TXE: u32 = 1 << 1;
const RXNE: u32 = 1 << 0;

pub mod spi_registers {
    pub const spi_cr1: *mut usize = SPI_CR1 as *mut usize;
    pub const spi_cr2: *mut usize = SPI_CR2 as *mut usize;
    pub const spi_sr: *mut usize = SPI_SR as *mut usize;
    pub const spi_dr1: *mut usize = SPI_DR1 as *mut usize;
    pub const spi_dr2: *mut usize = SPI_DR2 as *mut usize;
    pub const spi_dr3: *mut usize = SPI_DR3 as *mut usize;
    pub const spi_dr4: *mut usize = SPI_DR4 as *mut usize;
    pub const spi_dr5: *mut usize = SPI_DR5 as *mut usize;
    pub const spi_crcpr: *mut usize = SPI_CRCPR as *mut usize;
    pub const spi_rxcrcr: *mut usize = SPI_RXCRCR as *mut usize;
    pub const spi_txcrcr: *mut usize = SPI_TXCRCR as *mut usize;
}

pub fn flash_cmd_addr(command: u32, addr: u32) -> u32 {
    let address1 = bit_extracted(addr, 24, 9);
    let address2 = bit_extracted(addr, 8, 1);
    let data1 = command | address1;
    let address2 = address2 << 24;

    set_spi(spi_dr1, data1);
    set_spi(spi_dr2, address2);
    set_spi(spi_dr5, 0);
    set_spi(
        spi_cr1,
        SPI_BR(7) | SPI_TOTAL_BITS_TX(40) | SPI_TOTAL_BITS_RX(0) | SPI_SPE | SPI_CPHA | SPI_CPOL,
    );
    wait_for(20);
    spi_not_busy();

    1
}

pub fn set_spi(addr: *mut u32, val: u32) {
    unsafe {
        *addr = val;
    }
}

pub fn spi_not_busy() -> u32 {
    let mut value = 0x80;
    while value & 0x80 != 0 {
        wait_for(10);
        value = get_spi(spi_sr);
    }
    1
}

pub fn bit_extracted(value: u32, start_bit: u32, num_bits: u32) -> u32 {
    ((value >> start_bit) & ((1 << num_bits) - 1))
}

pub fn get_spi(addr: *const u32) -> u32 {
    unsafe {
        *addr
    }
}

pub fn wait_for(duration: u32) {
    // Your wait implementation here
}

pub fn spi_init() {
    set_spi(spi_cr1, SPI_BR(7) | SPI_CPHA | SPI_CPOL);
}

pub fn configure_spi(offset: usize) {
    spi_cr1 = (SPI_CR1 + offset) as *mut u32;
    spi_cr2 = (SPI_CR2 + offset) as *mut u32;
    spi_sr = (SPI_SR + offset) as *mut u32;
    spi_dr1 = (SPI_DR1 + offset) as *mut u32;
    spi_dr2 = (SPI_DR2 + offset) as *mut u32;
    spi_dr3 = (SPI_DR3 + offset) as *mut u32;
    spi_dr4 = (SPI_DR4 + offset) as *mut u32;
    spi_dr5 = (SPI_DR5 + offset) as *mut u32;
    spi_crcpr = (SPI_CRCPR + offset) as *mut u32;
    spi_rxcrcr = (SPI_RXCRCR + offset) as *mut u32;
    spi_txcrcr = (SPI_TXCRCR + offset) as *mut u32;
}



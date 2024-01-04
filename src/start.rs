use core::arch::global_asm;
use riscv::asm;
use volatile_register::RW;

// Assembly counterpart to this file.
global_asm!(include_str!("trap.S"));
global_asm!(include_str!("start.S"));


#[repr(C)]
struct Mtimecmp {
    value: RW<u64>,
}

pub const MTIMECMP: *mut Mtimecmp = 0x02004000 as *mut Mtimecmp;

pub const PLIC_BASE_ADDRESS: u32 = 0x0C000000; // /*! PLIC Interface Start */
pub const PLIC_CLAIM_OFFSET: u32 = 0x10010; 

pub fn mach_clint_handler(_int_id: usize, _epc: usize) {
    crate::vajra::write_uart_string("\n mach_clint_handler entered".as_ptr());

    // Assuming `mtimecmp` is a mutable static pointer or some accessible memory location

    // Set mtimecmp to some value. On application request basis handle timer interrupt
    // unsafe {
    //     *MTIMECMP = -1; // Setting it to usize::MAX as an example; replace with the desired value
    // }

    crate::vajra::write_uart_string("Timer interrupt handled".as_ptr());

    crate::vajra::write_uart_string("mach_clint_handler exited".as_ptr());
}

pub fn mach_plic_handler(_int_id: usize, _epc: usize) {
    let interrupt_id:u32;

    crate::vajra::write_uart_string("\nmach_plic_handler entered".as_ptr());

    interrupt_id = interrupt_claim_request();

    crate::vajra::write_uart_string("\n interrupt id claimed ".as_ptr());

    // if (interrupt_id <= 0 || interrupt_id > PLIC_MAX_INTERRUPT_SRC)
	// {
	// 	log_fatal("Fatal error, interrupt id [%x] claimed is wrong\n", interrupt_id);
	// }

}


pub fn interrupt_claim_request() -> u32 {
    let mut interrupt_claim_address: *mut u32 = core::ptr::null_mut();
    let mut interrupt_id: u32;

    unsafe {
        crate::vajra::write_uart_string("\ninterrupt_claim_request entered".as_ptr());

        // Calculate the interrupt claim address
        interrupt_claim_address = (PLIC_BASE_ADDRESS + PLIC_CLAIM_OFFSET) as *mut u32;

        // Claim the interrupt by reading from the memory-mapped address
        interrupt_id = *interrupt_claim_address;

        crate::vajra::write_uart_string(
            "interrupt id  claimed at address ".as_ptr());

        crate::vajra::write_uart_string("interrupt_claim_request exited".as_ptr());
    }

    interrupt_id
}

#[derive(Debug, PartialEq)]
pub enum Interrupt {
    UserSoftware,
    SupervisorSoftware,
    ReservedInterrupt0,
    MachineSoftware,
    UserTimer,
    SupervisorTimer,
    ReservedInterrupt1,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    ReservedInterrupt2,
    MachineExternal,
    ReservedInterrupt3,
    ReservedInterrupt4,
    ReservedInterrupt5,
    ReservedInterrupt6,
}

#[derive(Debug, PartialEq)]
pub enum Trap {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAmoAddressMisaligned,
    StoreAmoAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    ReservedTrap1,
    EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    ReservedTrap2,
    StoreAmoPageFault,
}


fn trap_init() {
    
    crate::vajra::write_uart_string("trap_init entered".as_ptr());

    unsafe {
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::UserSoftware as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::SupervisorSoftware as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::ReservedInterrupt0 as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::MachineSoftware as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::UserTimer as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::SupervisorTimer as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::ReservedInterrupt1 as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::MachineTimer as usize] = mach_clint_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::UserExternal as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::SupervisorExternal as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::ReservedInterrupt2 as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::MachineExternal as usize] = mach_plic_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::ReservedInterrupt3 as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::ReservedInterrupt4 as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::ReservedInterrupt5 as usize] = crate::trap::default_handler;
        crate::trap::MCAUSE_INTERRUPT_TABLE[Interrupt::ReservedInterrupt6 as usize] = crate::trap::default_handler;

        // crate::trap::MCAUSE_TRAP_TABLE[Trap::INSTRUCTION_ADDRESS_MISALIGNED as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::INSTRUCTION_ACCESS_FAULT as usize] =crate::trap:: default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::ILLEGAL_INSTRUCTION as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::BREAKPOINT as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::LOAD_ADDRESS_MISALIGNED as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::LOAD_ACCESS_FAULT as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::STORE_AMO_ADDRESS_MISALIGNED as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::STORE_AMO_ACCESS_FAULT as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::ENVIRONMENT_CALL_FROM_U_MODE as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::ENVIRONMENT_CALL_FROM_S_MODE as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::RESERVED_TRAP1 as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::ENVIRONMENT_CALL_FROM_M_MODE as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::INSTRUCTION_PAGE_FAULT as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::LOAD_PAGE_FAULT as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::RESERVED_TRAP2 as usize] = crate::trap::default_handler;
        // crate::trap::MCAUSE_TRAP_TABLE[Trap::STORE_AMO_PAGE_FAULT as usize] = crate::trap::default_handler;
    }

    crate::vajra::write_uart_string("trap_init exited".as_ptr());
}



/// The Rust entry of the  binary.
///
/// The function is called from the assembly `_start` function.
///
#[no_mangle]
pub unsafe extern "C" fn init() -> ! {

    crate::vajra::uart_init();
    
    crate::vajra::write_uart_string("init entered".as_ptr());

    #[cfg(feature = "AARDONYX")]
    {
        micron_disable_xip_volatile(0, 0);
        flash_mem_init();
    }

    // trap_init();

    crate::start_shakti();
    
    crate::vajra::write_uart_string("init exited".as_ptr());
    loop {
        
    }
}

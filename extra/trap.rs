use core::{ptr, arch::global_asm};

const MAX_TRAP_VALUE: usize = 16/* your value */;
const MAX_INTERRUPT_VALUE: usize = 16 ;

type MTrapFptrT = fn(usize, usize);
// global_asm!(include_str!("trap.S"));

pub static mut MCAUSE_TRAP_TABLE: [MTrapFptrT; MAX_TRAP_VALUE] = [default_handler; MAX_TRAP_VALUE];
pub static mut MCAUSE_INTERRUPT_TABLE: [MTrapFptrT; MAX_INTERRUPT_VALUE] = [default_handler; MAX_INTERRUPT_VALUE];

/// default handler that loops infinitely
pub fn default_handler(_mcause: usize, _epc: usize) {
    crate::vajra::write_uart_string("\ndefault_handler entered\n".as_ptr());

    // Using a loop to simulate the infinite loop
    loop {
        unsafe {
            // Use ptr::null() to simulate the equivalent of a null pointer in C
            // You might want to replace this with the appropriate handling
            // based on the Rust logic of your application
            let _ = ptr::null::<u8>();
        }
    }

    crate::vajra::write_uart_string("default_handler exited\n".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn handle_trap(_mcause: usize, _epc:usize) -> usize {
    crate::vajra::write_uart_string("\nHandle trap entered\n".as_ptr());
    let __riscv_xlen:usize = 64;
    let shift_length = __riscv_xlen - 1;

    if _mcause & (1 << shift_length) != 0 {
        let ie_entry = extract_ie_code(_mcause);
        crate::vajra::write_uart_string("\nInterrupt: mcause = {:x}, epc = {:x}\n".as_ptr());
        MCAUSE_INTERRUPT_TABLE[ie_entry](_mcause, _epc);
    } else {
        crate::vajra::write_uart_string("\nException: mcause = {:x}, epc = {:x}\n".as_ptr());
        MCAUSE_TRAP_TABLE[_mcause](_mcause, _epc);
    }
    _epc
}

fn extract_ie_code(num: usize) -> usize {
    crate::vajra::write_uart_string("\nextract_ie_code entered\n".as_ptr());

    let exception_code = num & 0x7FFFFFFF;

    crate::vajra::write_uart_string("exception code = {:x}\n".as_ptr());

    crate::vajra::write_uart_string("extract_ie_code exited\n".as_ptr());

    exception_code
}


use core::ptr;

const MAX_TRAP_VALUE: usize = 16/* your value */;
const MAX_INTERRUPT_VALUE: usize = 16 ;

type MTrapFptrT = fn(usize, usize);

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

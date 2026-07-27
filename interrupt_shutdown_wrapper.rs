use core::arch::{asm, naked_asm};
use crate::shutdown;
use crate::push_all_registers;
use crate::pop_all_registers;

#[no_mangle]

pub fn shutdown_driver() {

    unsafe {

        asm!("cli");
        shutdown::shutdown();
        asm!("sti");
    
    }

}

#[unsafe(naked)]

pub extern "C" fn interrupt_shutdown() {

        naked_asm!(
                push_all_registers!(),
                "call shutdown_driver",
                pop_all_registers!(),
                "iretq"
        );

}

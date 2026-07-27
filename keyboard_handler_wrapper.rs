use core::arch::{asm, naked_asm};
use crate::apic_read;
use crate::push_all_registers;
use crate::pop_all_registers;
use crate::keyboard;

#[no_mangle]

pub extern "C" fn keyboard_handler() {

    unsafe {

        asm!("cli");
        keyboard::keyboard_read();
        asm!("sti");

        apic_read::apic_write(apic_read::REG_EOI as usize, 0 as usize);
    
    }

}

#[unsafe(naked)]

pub extern "C" fn interrupt_keyboard_handler() {

        naked_asm!(
                push_all_registers!(),
                "call keyboard_handler",
                pop_all_registers!(),
                "iretq"
        );

}

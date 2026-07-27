use crate::GLOBALS;
use core::arch::naked_asm;
use crate::apic_read;
use crate::push_all_registers;
use crate::pop_all_registers;
use crate::print_screen;

#[no_mangle]

pub extern "C" fn interrupt_is_ready() {

    unsafe {

        print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 0, 95, 0xFFFFFF, "[+] Interrupts is ready.");
        apic_read::apic_write(apic_read::REG_EOI as usize, 0 as usize);
    
    }

}

#[unsafe(naked)]

pub extern "C" fn call_interrupt() {

        naked_asm!(
                push_all_registers!(),
                "call interrupt_is_ready",
                pop_all_registers!(),
                "iretq"
        );

}

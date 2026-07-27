use crate::idt::HELP_CMD;
use crate::GLOBALS;
use crate::print_screen;
use crate::push_all_registers;
use crate::pop_all_registers;
use core::arch::naked_asm;

#[no_mangle]

pub extern "C" fn help_interrupt() {

    unsafe {

        if HELP_CMD >= GLOBALS.global_stride || HELP_CMD >= GLOBALS.global_with || HELP_CMD >= GLOBALS.global_height {
                HELP_CMD = 0;
        }

        print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 0, HELP_CMD, 0xFFFFFF, "[+] int 36: Show value of ECX register. int 37: Shutdown System. int 38: For clearing screen and terminal color. int 39: For skull.");
    
    }

}

#[unsafe(naked)]

pub extern "C" fn interrupt_help() {

        naked_asm!(
                push_all_registers!(),
                "call help_interrupt",
                pop_all_registers!(),
                "iretq"
        );

}

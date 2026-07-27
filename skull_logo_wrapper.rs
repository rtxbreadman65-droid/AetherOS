use crate::skull;
use crate::GLOBALS;
use crate::push_all_registers;
use crate::pop_all_registers;
use core::arch::{asm, naked_asm};

#[no_mangle]

pub extern "C" fn skull_logo() {

    unsafe {

        asm!("cli");
        skull::skull(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 320, 250);
        asm!("sti");
    
    }

}

#[unsafe(naked)]

pub extern "C" fn interrupt_skull_logo() {

        naked_asm!(
                push_all_registers!(),
                "call skull_logo",
                pop_all_registers!(),
                "iretq"
        );

}

use crate::idt::CURRENT_COLOR;
use crate::clear_screen;
use crate::idt::HELP_CMD;
use crate::keyboard::CURRENT_POSITION;
use crate::keyboard::CURRENT_LINE;
use crate::GLOBALS;
use core::arch::{asm, naked_asm};
use crate::push_all_registers;
use crate::pop_all_registers;

#[no_mangle]
pub extern "C" fn color_change() {

    unsafe { 

        asm!("cli"); 

        if CURRENT_COLOR >= 2 {

            clear_screen::clear_screen(GLOBALS.global_fb_ptr, GLOBALS.global_with, GLOBALS.global_height, 0x000080);
            HELP_CMD = 0;
            CURRENT_POSITION = 50;
            CURRENT_LINE = 0;
            CURRENT_COLOR = 0;
    
        }

        else {

            clear_screen::clear_screen(GLOBALS.global_fb_ptr, GLOBALS.global_with, GLOBALS.global_height, 0x000000);
            HELP_CMD = 0;
            CURRENT_POSITION = 50;
            CURRENT_LINE = 0;
    
        }
    
        CURRENT_COLOR += 1;
        asm!("sti");

    }

}

#[unsafe(naked)]

pub extern "C" fn interrupt_color_change() {

        naked_asm!(
                push_all_registers!(),
                "call color_change",
                pop_all_registers!(),
                "iretq"
        );

}

use core::arch::naked_asm;
use crate::GLOBALS;
use core::arch::asm;
use crate::idt::HELP_CMD;
use crate::keyboard::INPUT_BUFFER;
use crate::print_screen;

#[inline(never)]
#[no_mangle]
pub extern "C" fn invalid_opcode_handler() {

    unsafe {

        print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 0, HELP_CMD, 0xFFFFFF, "[!] INVALID OPCODE.");
        INPUT_BUFFER.fill(0);
        INPUT_BUFFER[0] = 0x90;
        INPUT_BUFFER[1] = 0x90;
        INPUT_BUFFER[2] = 0xC3;

        asm!(
        	"mov [rsp + 16], {0}",
        	in(reg) INPUT_BUFFER.as_ptr()
        );

        asm!(
        	"mov rax, [rsp + 16]",
        	"call rax"
        );

    }
	
}

#[unsafe(naked)]
pub extern "C" fn invalid_opcode_handler_wrapper() {

    naked_asm!(
    	"call invalid_opcode_handler",
    	"iretq"
    );
	
}

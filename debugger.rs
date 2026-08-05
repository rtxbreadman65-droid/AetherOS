use crate::GLOBALS;
use crate::print_screen;
use core::arch::naked_asm;
use crate::idt::HELP_CMD;
use crate::hex_converter;
use crate::push_all_registers;
use crate::pop_all_registers;

#[no_mangle]
pub extern "C" fn debugger(val: u64) {

    unsafe {

        let stack = val as *const u64;
        let rip = *stack;

        let mut buf = [0u8; 16];
        hex_converter::hex_converter(rip, &mut buf);
        let hex_str: &str = core::str::from_utf8_unchecked(&buf);

        print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 0, HELP_CMD, 0xFFFFFF, "RIP : ");
        print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 60, HELP_CMD, 0xFFFFFF, hex_str);

    }
	
}

#[unsafe(naked)]
pub extern "C" fn debugger_handler() {

    naked_asm!(
        push_all_registers!(),
    	"lea rdi, [rsp + 72]",
    	"call debugger",
    	pop_all_registers!(),
    	"iretq"
    );
	
}

use crate::print_screen;
use crate::idt::HELP_CMD;
use crate::push_all_registers;
use crate::pop_all_registers;
use core::arch::naked_asm;
use crate::GLOBALS;
use crate::hex_converter;

#[no_mangle]

pub extern "C" fn register_value_read(stack_pointer: *mut u64) {


    unsafe {

        let register_rcx = *stack_pointer.add(7);
        let register_ecx = register_rcx as u32;

        let mut buf = [0u8; 16];
        hex_converter::hex_converter(register_ecx as u64, &mut buf);

        let hex_string: &str = core::str::from_utf8_unchecked(&buf);
        print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 0, HELP_CMD, 0xFFFFFF, "ECX -");
        print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 50, HELP_CMD, 0xFFFFFF, hex_string);

    }

}

#[unsafe(naked)]

pub extern "C" fn interrupt_registers_read() {

        naked_asm!(
            //"mov ecx, 0x11223344",
                push_all_registers!(),
                "mov rdi, rsp",
                "call register_value_read",
                pop_all_registers!(),
                "iretq"
        );

}

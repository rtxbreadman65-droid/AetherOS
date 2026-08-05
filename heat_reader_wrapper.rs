use crate::GLOBALS;
use crate::push_all_registers;
use crate::pop_all_registers;
use core::arch::{naked_asm, asm};
use crate::heat_reader;
use crate::print_screen;
use crate::u32_converter;
use crate::apic_read;
use crate::patch_screen;

static mut CURRENT_TIMER: usize = 0 as usize;

#[no_mangle]

pub extern "C" fn heat_reader() {

    unsafe {

        CURRENT_TIMER += 1;

        asm!("cli");
        let temp = heat_reader::heatreader();
        
        let mut buf = [0u8; 20];
        let real_temp = u32_converter::convert_str(temp.into(), &mut buf);

        print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, (GLOBALS.global_height - 150) as usize, 0, 0xFFFFFF, "CPU Temp:");
        print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, (GLOBALS.global_height - 50) as usize, 0, 0xFFFFFF, real_temp);
        
        if CURRENT_TIMER >= 2 {
                patch_screen::patch_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, (GLOBALS.global_height - 150) as usize, 0, 0x000000, 130);
                CURRENT_TIMER = 0;
        }
        
        asm!("sti");

        apic_read::apic_write(apic_read::REG_EOI as usize, 0 as usize);
    }

}

#[unsafe(naked)]

pub extern "C" fn interrupt_heat_reader() {
    
        naked_asm!(
                push_all_registers!(),
                "call heat_reader",
                pop_all_registers!(),
                "iretq"
        );

}



use core::arch::asm;
use crate::GLOBALS;
use crate::TEMPOLINE_CODE;
use crate::apic_read;
use crate::print_screen;
use crate::clear_screen;

pub extern "C" fn core_1_active() {

    let global_color: u32 = 0xFFFFFF as u32;

    unsafe {

        let mut cr3_val: u64 = 0 as u64;

        asm!(
                "mov rax, cr3",
                out("rax") cr3_val,
        );

        core::ptr::write(0x6000 as *mut u64, cr3_val);
    
    }

    unsafe {

        let tempoline_code_address: *mut u8 = 0x8000 as *mut u8;
        let raw_address = TEMPOLINE_CODE.as_ptr();

        core::ptr::copy_nonoverlapping(raw_address, tempoline_code_address, TEMPOLINE_CODE.len());

        apic_read::apic_write(0x310 as usize, (2 << 24) as usize);
        apic_read::apic_write(0x300 as usize, 0x00004500 as usize);

        for _ in 0..20_000_000_u64 {
                asm!("pause");
        }
        
        apic_read::apic_write(0x310 as usize, (2 << 24) as usize);
        apic_read::apic_write(0x300 as usize, (0x00004600 | 0x08) as usize);

        core::ptr::write(0x7000 as *mut u64, 0 as u64);

        let mut sipi_count: usize = 0 as usize;

        loop {

            if sipi_count > 5 {
                break;
            }

            for _ in 0..10_000_000_u64 {
                asm!("pause");
            }

            let results = core::ptr::read_volatile(0x7000 as *const u8);

            if results == 5 {
                    break;
            }

            else {
                apic_read::apic_write(0x310 as usize, (2 << 24) as usize);
                apic_read::apic_write(0x300 as usize, (0x00004600 | 0x08) as usize);
            }

            sipi_count += 1;

        };

        for _ in 0..20_000_000_u64 {
                asm!("pause");
        }

        clear_screen::clear_screen(GLOBALS.global_fb_ptr, GLOBALS.global_with, GLOBALS.global_height, 0x000000);
    
    };

    unsafe {

        let results = core::ptr::read_volatile(0x7000 as *const u8);
    
        if results == 5 {
            print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 0, 145, global_color, "[+] Core 1 is active.");
            return;
        }

        else {
            print_screen::print_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 0, 145, global_color, "[+] Failed to active core 1.");
            return;
        }

     }

	
}

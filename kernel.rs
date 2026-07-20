#![no_std]
#![no_main]

use core::arch::asm;
mod logo;
mod clear_screen;
mod introduction_to_kernel;
mod font;
mod print;
mod print_screen;
mod acpi_read;
mod pc_driver;
mod heat_reader;
mod u32_converter;
mod check_chip_status;
mod gdt;
mod idt;
mod shutdown;
mod pic_disable;
mod apic_read;
mod KEYBOARD_MAP;
mod patch_screen;
mod keyboard;
mod ioapic;
mod hex_converter;

static mut help_cmd: usize = 0 as usize;

//pub static mut CORE_1_READY: usize = 0 as usize;
//pub static ASM_PAYLOAD: &[u8] = include_bytes!("asm_opcode_review.bin");
//pub static PRINT_SCREEN: &[u8] = include_bytes!("print_screen_asm.bin");
pub static TEMPOLINE_CODE: &[u8] = include_bytes!("tempoline_code.bin");

#[repr(C, packed)]
pub struct GLOBAL_VARIABLES {
	pub GLOBAL_FB_PTR: *mut u32,
	pub GLOBAL_STRIDE: usize,
	pub GLOBAL_WITH: usize,
	pub GLOBAL_HEIGHT: usize,
	pub RDSP: usize
}

pub static mut GLOBALS: GLOBAL_VARIABLES = GLOBAL_VARIABLES {
	GLOBAL_FB_PTR: core::ptr::null_mut(),
	GLOBAL_STRIDE: 0,
	GLOBAL_WITH: 0,
	GLOBAL_HEIGHT: 0,
	RDSP: 0
};

#[no_mangle]
#[link_section = ".text_start"]

pub extern "C" fn kernel_main(fb_ptr: *mut u32, screen_with: usize, screen_height: usize, stride: usize, rdsp: usize) -> ! {

    unsafe {
        GLOBALS.GLOBAL_FB_PTR = fb_ptr;
        GLOBALS.GLOBAL_STRIDE = stride;
        GLOBALS.GLOBAL_WITH = screen_with;   
        GLOBALS.GLOBAL_HEIGHT = screen_height;
        GLOBALS.RDSP = rdsp;
    }

    unsafe {

        asm!(
    	    "mov rax, cr4",
    	    "and rax, ~(1 << 20 )",
    	    "mov cr4, rax",
    	    options(nostack, preserves_flags)
        );

    }

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

        loop {

            for _ in 0..10_000_000_u64 {
            	asm!("pause");
            }

            let results = core::ptr::read_volatile(0x7000 as *const u8);

            if results == 5 {
        	    print_screen::print_screen(fb_ptr, stride, 0, 0, 0x00FF00, "[+] core 1 is active.");
        	    break;
            }

            else {
            	apic_read::apic_write(0x310 as usize, (2 << 24) as usize);
            	apic_read::apic_write(0x300 as usize, (0x00004600 | 0x08) as usize);
            }

        }

        for _ in 0..20_000_000_u64 {
        	asm!("pause");
        }

        clear_screen::clear_screen(fb_ptr, screen_with, screen_height, 0x000000);
    	
    }

    unsafe { logo::logo(fb_ptr, stride, 320, 250); }

    for _ in 0..20_000_000_u64 {
    	unsafe { asm!("pause"); }
    }

    clear_screen::clear_screen(fb_ptr, screen_with, screen_height, 0x000000);

    print_screen::print_screen(fb_ptr, stride, 0, 0, 0x00FF00, "[+] Welcome to AetherOS.");
    print_screen::print_screen(fb_ptr, stride, 0, 15, 0x00FF00, "[+] Kernel booted successfully.");
    print_screen::print_screen(fb_ptr, stride, 0, 30, 0x00FF00, "[+] This OS is made by Arshman Farhan.");
    print_screen::print_screen(fb_ptr, stride, 0, 45, 0x00FF00, "[+] Setting up Global Descripter Table (GDT).");
    gdt::load_gdt();
    print_screen::print_screen(fb_ptr, stride, 0, 60, 0x00FF00, "[+] GDT settings done.");
    print_screen::print_screen(fb_ptr, stride, 0, 77, 0x00FF00, "[+] Setting up IDT entries.");
    idt::set_idt_gate(32, idt::call_interrupt as u64);
    idt::set_idt_gate(33, idt::interrupt_heat_reader as u64);
    idt::set_idt_gate(34, idt::interrupt_keyboard_handler as u64);
    idt::set_idt_gate(35, idt::interrupt_help as u64);
    idt::set_idt_gate(36, idt::interrupt_registers_read as u64);
    idt::set_idt_gate(37, idt::interrupt_shutdown as u64);
    idt::idt_load();

    unsafe {
    	asm!("sti");
    	asm!("int 32");
    }
    unsafe { pic_disable::disable_pic(); }
    print_screen::print_screen(fb_ptr, stride, 0, 130, 0x00FF00, "[+] initializing keyboard driver.");
    ioapic::keyboard_ioapic_init();

    for _ in 0..20_000_000_u64 {
    	unsafe {
    		asm!("pause");
    	}
    }
    clear_screen::clear_screen(fb_ptr, screen_with, screen_height, 0x000000);

    print_screen::print_screen(fb_ptr, stride, 0, 0, 0x00FF00, "ROOT#");

    apic_read::init_apic();
    apic_read::set_timer(100000, 33);
    

    loop {
        unsafe {
        	asm!("hlt");
        }
    }

}

#[panic_handler]

unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {

    //unsafe { pc_driver::pc_speaker(); }
    clear_screen::clear_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_WITH, GLOBALS.GLOBAL_HEIGHT, 0x00FFFF);
    print::print(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 0, 0x00FF00, "[!] KERNEL PANIC KERNEL PANIC KERNEL PANIC");

    loop {
    	unsafe {
    		asm!("hlt");
    	}
    }
}

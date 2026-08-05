#![no_std]
#![no_main]

use core::arch::asm;
mod logo;
mod clear_screen;
mod font;
mod print;
mod print_screen;
mod pc_driver;
mod heat_reader;
mod u32_converter;
mod gdt;
mod heat_reader_wrapper;
mod idt;
mod shutdown;
mod pic_disable;
mod apic_read;
mod keyboard_map;
mod patch_screen;
mod keyboard;
mod ioapic;
mod hex_converter;
mod skull;
mod interrupt_is_ready;
mod core_1_active;
mod color_changer;
mod keyboard_handler_wrapper;
mod help_interrupt_wrapper;
mod interrupt_shutdown_wrapper;
mod interrupt_register_read_wrapper;
mod skull_logo_wrapper;
mod invalid_op_code_handler;
mod debugger;

pub static TEMPOLINE_CODE: &[u8] = include_bytes!("tempoline_code.bin");

#[repr(C, packed)]
pub struct GLOBAL_VARIABLES {
	pub global_fb_ptr: *mut u32,
	pub global_stride: usize,
	pub global_with: usize,
	pub global_height: usize,
	pub rdsp: usize
}

pub static mut GLOBALS: GLOBAL_VARIABLES = GLOBAL_VARIABLES {
	global_fb_ptr: core::ptr::null_mut(),
	global_stride: 0,
	global_with: 0,
	global_height: 0,
	rdsp: 0
};

#[no_mangle]
#[link_section = ".text_start"]

pub extern "C" fn kernel_main(fb_ptr: *mut u32, screen_with: usize, screen_height: usize, stride: usize, rdsp: usize) -> ! {

    let global_color: u32 = 0xFFFFFF as u32;

    unsafe {
        GLOBALS.global_fb_ptr = fb_ptr;
        GLOBALS.global_stride = stride;
        GLOBALS.global_with = screen_with;   
        GLOBALS.global_height = screen_height;
        GLOBALS.rdsp = rdsp;
    }

    unsafe {

        asm!(
    	    "mov rax, cr4",
    	    "and rax, ~(1 << 20 )",
    	    "mov cr4, rax",
    	    options(nostack, preserves_flags)
        );

    }

    unsafe { logo::logo(fb_ptr, stride, 320, 250); }

    core_1_active::core_1_active();

    print_screen::print_screen(fb_ptr, stride, 0, 0, global_color, "[+] Welcome to AetherOS.");
    print_screen::print_screen(fb_ptr, stride, 0, 15, global_color, "[+] Kernel booted successfully.");
    print_screen::print_screen(fb_ptr, stride, 0, 30, global_color, "[+] This OS is made by Arshman Farhan.");
    print_screen::print_screen(fb_ptr, stride, 0, 45, global_color, "[+] Setting up Global Descripter Table (GDT).");
    gdt::load_gdt();
    print_screen::print_screen(fb_ptr, stride, 0, 60, global_color, "[+] GDT settings done.");
    print_screen::print_screen(fb_ptr, stride, 0, 77, global_color, "[+] Setting up IDT entries.");
    idt::set_idt_gate(3, debugger::debugger_handler as *const () as u64);
    idt::set_idt_gate(6, invalid_op_code_handler::invalid_opcode_handler_wrapper as *const () as u64);
    idt::set_idt_gate(32, interrupt_is_ready::call_interrupt as *const () as u64);
    idt::set_idt_gate(33, heat_reader_wrapper::interrupt_heat_reader as *const () as u64);
    idt::set_idt_gate(34, keyboard_handler_wrapper::interrupt_keyboard_handler as *const () as u64);
    idt::set_idt_gate(35, help_interrupt_wrapper::interrupt_help as *const () as u64);
    idt::set_idt_gate(36, interrupt_register_read_wrapper::interrupt_registers_read as *const () as u64);
    idt::set_idt_gate(37, interrupt_shutdown_wrapper::interrupt_shutdown as *const () as u64);
    idt::set_idt_gate(38, color_changer::interrupt_color_change as *const () as u64);
    idt::set_idt_gate(39, skull_logo_wrapper::interrupt_skull_logo as *const () as u64);
    idt::idt_load();

    unsafe {
    	asm!("sti");
    	asm!("int 32");
    }
    unsafe { pic_disable::disable_pic(); }
    print_screen::print_screen(fb_ptr, stride, 0, 130, global_color, "[+] initializing keyboard driver.");
    ioapic::keyboard_ioapic_init();

    for _ in 0..20_000_000_u64 {
    	unsafe {
    		asm!("pause");
    	}
    }
    clear_screen::clear_screen(fb_ptr, screen_with, screen_height, 0x000000);

    print_screen::print_screen(fb_ptr, stride, 0, 0, global_color, "ROOT#");
    
    apic_read::init_apic();
    apic_read::set_timer(100000, 33);
    

    //unsafe { asm!("int 3"); }
    loop {
        unsafe {
        	asm!("hlt");
        }
    }

}

#[panic_handler]

unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {

    unsafe { pc_driver::pc_speaker(); }
    clear_screen::clear_screen(GLOBALS.global_fb_ptr, GLOBALS.global_stride, GLOBALS.global_height, 0x00FFFF);
    print::print(GLOBALS.global_fb_ptr, GLOBALS.global_stride, 0, 0x00FF00, "[!] KERNEL PANIC KERNEL PANIC KERNEL PANIC");

    loop {
    	unsafe {
    		asm!("hlt");
    	}
    }
}

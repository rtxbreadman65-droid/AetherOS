use core::arch::{asm, naked_asm};
use crate::GLOBALS;
use crate::print_screen;
use crate::apic_read;
use crate::clear_screen;
use crate::heat_reader;
use crate::u32_converter;
use crate::patch_screen;
use crate::keyboard;
use crate::hex_converter;
use crate::shutdown;
use crate::keyboard::current_position;
use crate::keyboard::current_line;

pub static mut current_timer: usize = 0;
pub static mut help_cmd: usize = 0;
pub static mut current_color: usize = 0 as usize;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct idtEntry {
	pub handler_address_lower: u16,
	pub segment_selector: u16,
	pub interrupt_stack_table: u8,
	pub type_attribute: u8,
	pub handler_address_mid: u16,
	pub handler_address_high: u32,
	pub reserved: u32
}

pub static mut IDT: [idtEntry; 256] = [idtEntry {
	handler_address_lower: 0,
	segment_selector: 0,
	interrupt_stack_table: 0,
	type_attribute: 0,
	handler_address_mid: 0,
	handler_address_high: 0,
	reserved: 0
}; 256];

#[repr(C, packed)]
pub struct idtPointer {
	pub limit: u16, 
	pub base: u64
}

pub fn set_idt_gate(index: usize, handler_addr: u64) {

    unsafe {

        IDT[index].handler_address_lower = handler_addr as u16;
        IDT[index].handler_address_mid = (handler_addr >> 16) as u16;
        IDT[index].handler_address_high = (handler_addr >> 32) as u32;
        IDT[index].segment_selector = 0x08 as u16;
        IDT[index].interrupt_stack_table = 0 as u8;
        IDT[index].reserved = 0 as u32;
        IDT[index].type_attribute = 0x8E as u8;
    	
    }
	
}

pub fn idt_load() {

    let pointer = unsafe {
    	idtPointer {
    		limit: 4095,
    		base: &raw const IDT as u64
    	}
    };

    unsafe {
    	asm!(
    		"lidt [{0}]", in(reg) &pointer
    	);
    }
	
}

#[macro_export]
macro_rules! push_all_registers {

    () => {
    	concat!(
            "push rax\n",
            "push rcx\n",
            "push rdx\n",
            "push rsi\n",
            "push rdi\n",
            "push r8\n",
            "push r9\n",
            "push r10\n",
            "push r11\n"
        )
    };
	
}

#[macro_export]
macro_rules! pop_all_registers {

    () => {
        concat!(
            "pop r11\n",
            "pop r10\n",
            "pop r9\n",
            "pop r8\n",
            "pop rdi\n",
            "pop rsi\n",
            "pop rdx\n",
            "pop rcx\n",
            "pop rax\n"
        )
    };
}

#[no_mangle]

pub extern "C" fn interrupt_is_ready() {

    unsafe {

        print_screen::print_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 0, 95, 0xFFFFFF, "[+] Interrupts is ready.");
        apic_read::apic_write(apic_read::REG_EOI as usize, 0 as usize);
    	
    }
	
}

#[unsafe(naked)]

pub extern "C" fn call_interrupt() {

    unsafe {

        naked_asm!(
        	push_all_registers!(),
        	"call interrupt_is_ready",
        	pop_all_registers!(),
        	"iretq"
        );
    	
    }
	
}

#[no_mangle]

pub extern "C" fn heat_reader() {

    unsafe {

        current_timer += 1;

        asm!("cli");
        let temp = unsafe { heat_reader::heatreader() };
        
        let mut buf = [0u8; 20];
        let real_temp = unsafe { u32_converter::ConvertStr(temp.into(), &mut buf) };

        print_screen::print_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 1855, 0, 0xFFFFFF, "CPU Temp:");
        print_screen::print_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 1905, 0, 0xFFFFFF, real_temp);
        
        if current_timer >= 2 {
        	patch_screen::patch_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 1855, 0, 0x000000, 67);
        	current_timer = 0;
        }
        
        asm!("sti");

        apic_read::apic_write(apic_read::REG_EOI as usize, 0 as usize);
    }
	
}

#[no_mangle]
pub extern "C" fn color_change() {

    unsafe { 

        asm!("cli"); 

        if current_color >= 2 {

            clear_screen::clear_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_WITH, GLOBALS.GLOBAL_HEIGHT, 0x000080);
            help_cmd = 0;
            current_position = 30;
            current_line = 0;
            current_color = 0;
    	
        }

        else {

            clear_screen::clear_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_WITH, GLOBALS.GLOBAL_HEIGHT, 0x000000);
            help_cmd = 0;
            current_position = 30;
            current_line = 0;
    	
        }
    
        current_color += 1;
        asm!("sti");

    }
	
}

#[no_mangle]

pub extern "C" fn keyboard_handler() {

    unsafe {

        asm!("cli");
        keyboard::keyboard_read();
        asm!("sti");

        apic_read::apic_write(apic_read::REG_EOI as usize, 0 as usize);
    	
    }
	
}

#[no_mangle]

pub extern "C" fn help_interrupt() {

    unsafe {

        if help_cmd >= GLOBALS.GLOBAL_STRIDE || help_cmd >= GLOBALS.GLOBAL_WITH || help_cmd >= GLOBALS.GLOBAL_HEIGHT {
        	help_cmd = 0;
        }

        print_screen::print_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 0, help_cmd, 0xFFFFFF, "[+] int 36: Show value of ECX register. int 37: Shutdown System. int 38: For clearing screen and terminal color.");
        //help_cmd += 40;
    	
    }
	
}

#[no_mangle]

pub extern "C" fn register_value_read(stack_pointer: *mut u64) {


    unsafe {

        let register_rcx = *stack_pointer.add(7);
        let register_ecx = register_rcx as u32;

        let mut buf = [0u8; 16];
        hex_converter::hex_converter(register_ecx as u64, &mut buf);

        let hex_string: &str = core::str::from_utf8_unchecked(&buf);
        print_screen::print_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 0, help_cmd, 0xFFFFFF, "ECX -");
        print_screen::print_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 50, help_cmd, 0xFFFFFF, hex_string);

    }
	
}

#[no_mangle]

pub fn shutdown_driver() {

    unsafe {

        asm!("cli");
        shutdown::shutdown();
        asm!("sti");
    	
    }
	
}

#[unsafe(naked)]

pub extern "C" fn interrupt_heat_reader() {

    unsafe {
    
    	naked_asm!(
    		push_all_registers!(),
    		"call heat_reader",
    		pop_all_registers!(),
    		"iretq"
    	);
    }
	
}

#[unsafe(naked)]

pub extern "C" fn interrupt_keyboard_handler() {

    unsafe {

        naked_asm!(
        	push_all_registers!(),
        	"call keyboard_handler",
        	pop_all_registers!(),
        	"iretq"
        );
    	
    }
	
}

#[unsafe(naked)]

pub extern "C" fn interrupt_help() {

    unsafe {

        naked_asm!(
        	push_all_registers!(),
        	"call help_interrupt",
        	pop_all_registers!(),
        	"iretq"
        );
    	
    }
	
}

#[unsafe(naked)]

pub extern "C" fn interrupt_registers_read() {

    unsafe {

        naked_asm!(
            //"mov ecx, 0x11223344",
        	push_all_registers!(),
        	"mov rdi, rsp",
        	"call register_value_read",
        	pop_all_registers!(),
        	"iretq"
        );
    	
    }
	
}

#[unsafe(naked)]

pub extern "C" fn interrupt_shutdown() {

    unsafe {

        naked_asm!(
        	push_all_registers!(),
        	"call shutdown_driver",
        	pop_all_registers!(),
        	"iretq"
        );
    	
    }
	
}

#[unsafe(naked)]

pub extern "C" fn interrupt_color_change() {

    unsafe {

        naked_asm!(
        	push_all_registers!(),
        	"call color_change",
        	pop_all_registers!(),
        	"iretq"
        );
    	
    }
	
}

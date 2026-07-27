use core::arch::asm;

pub static mut HELP_CMD: usize = 0;
pub static mut CURRENT_COLOR: usize = 0 as usize;

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

    let pointer = idtPointer {
    		limit: 4095,
    		base: &raw const IDT as u64
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

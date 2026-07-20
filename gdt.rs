use asm;

#[repr(C, packed)]
pub struct gdt_struct {
	pub null: u64,
	pub kernel_segment: u64,
	pub kernel_data: u64
}

pub static mut gdt_data: gdt_struct = gdt_struct {
	null: 0x0 as u64,
        kernel_segment: 0x00209A0000000000 as u64,
        kernel_data: 0x0000920000000000 as u64 
};

#[repr(C, packed)]
pub struct gdt_ptr {
	pub limit: u16,
	pub base: u64
}

pub fn load_gdt() {

        let pointer = unsafe {
        
            gdt_ptr {
    	        limit: 23,
    	        base: &raw const gdt_data as u64,
        }
        
    };

    unsafe {
    
    	asm!(
    	     "lgdt [{0}]", 
    	     "mov ax, 0x10",
    	     "mov ds, ax",
    	     "mov es, ax",
    	     "mov fs, ax",
    	     "mov gs, ax",
    	     "mov ss, ax",
    	     "push 0x08",
    	     "lea rax, [rip + 2f]",
    	     "push rax",
    	     "retfq",
    	     "2:",
    	     in(reg) &pointer
    	 );

    }

}

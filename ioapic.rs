use core::arch::asm;

pub const IOAPIC_REGSEL: u32 = 0xFEC00000;
pub const IOAPIC_IOWIN: u32 = 0xFEC00010;

#[no_mangle]
pub unsafe fn ioapic_write(reg: u32, value: u32) {

    core::ptr::write_volatile(IOAPIC_REGSEL as *mut u32, reg);
    core::ptr::write_volatile(IOAPIC_IOWIN as *mut u32, value);
	
}

#[no_mangle]
pub fn keyboard_ioapic_init() {

    unsafe {

        ioapic_write(0x12 as u32, 34 as u32);
        ioapic_write(0x13 as u32, 0 as u32);
    	
    }
	
}

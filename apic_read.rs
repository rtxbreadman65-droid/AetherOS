use core::arch::{naked_asm, asm};

pub const APIC_BASE: u32 = 0xFEE00000;
pub const REG_SIV: u8 = 0xF0;
pub const REG_TIMER: u16 = 0x320;
pub const REG_DIV: u16 = 0x3E0;
pub const REG_INIT_COUNT: u16 = 0x380;
pub const REG_EOI: u8 = 0xB0;


pub fn apic_write(offset: usize, value: usize) {

    unsafe {

    let base_address = APIC_BASE as *mut u8;
    let final_address = (base_address.wrapping_add(offset)) as *mut u32;

    let write = core::ptr::write_volatile(final_address, value as u32);

    }

}

pub fn init_apic() {

    unsafe {

        let enable_bit_value = 0b000100000000;
        let vector_number = 0b11111111;

        apic_write(REG_SIV as usize, enable_bit_value | vector_number);
    	
    }
	
}

pub fn set_timer(timer: usize, vector_nb: u32) {

    unsafe {

        let vector_number = vector_nb;
        let periodic_table: u32 = 0x00020000;
        let combine_value = vector_nb | periodic_table;
        apic_write(REG_TIMER as usize, combine_value as usize);

        let tick_speed = 0x3;
        apic_write(REG_DIV as usize, tick_speed as usize);

        let count_down_value = timer;
        apic_write(REG_INIT_COUNT as usize, count_down_value as usize);
    	
    }
	
}

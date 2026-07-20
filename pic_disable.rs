use core::arch::asm;
use crate::print_screen;
use crate::GLOBALS;

#[repr(C, packed)]
pub struct DATA_PORTS;

impl DATA_PORTS {
	pub const MASTER_PIC_DATA_PORT: u16 = 0b00100001;
	pub const SLAVE_PIC_DATA_PORT: u16 = 0b10100001;
	pub const ALL_BIT_OFF: u8 = 0b11111111;
}

pub unsafe fn disable_pic() {

    asm!(
    	"out dx, al",
    	in("dx") DATA_PORTS::MASTER_PIC_DATA_PORT,
    	in("al") DATA_PORTS::ALL_BIT_OFF
    );

    asm!(
    	"out dx, al",
    	in("dx") DATA_PORTS::SLAVE_PIC_DATA_PORT,
    	in("al") DATA_PORTS::ALL_BIT_OFF
    );

    print_screen::print_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 0, 115, 0x00FF00, "[+] PIC disabled successfully.");
	
}

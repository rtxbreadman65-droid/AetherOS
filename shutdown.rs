use core::arch::asm;
use core::ops::Add;
use crate::print_screen;
use crate::GLOBALS;
use idt::help_cmd;

pub fn shutdown() {

    unsafe {

        let fb_ptr = GLOBALS.GLOBAL_FB_PTR;
        let stride = GLOBALS.GLOBAL_STRIDE;
        let rdsp = GLOBALS.RDSP;

        let rdsp_u8 = rdsp as *const u8;
        let xsdt = (rdsp_u8.add(24) as *const u64).read();
        let xsdt_signature = (xsdt as *const u32).read();

        let total_lenght = xsdt.add(4);
        let lenght = (total_lenght as *const u32).read();
        let xsdt_entry = xsdt.add(36);
        let total_entries = (lenght - 36) / 8;

        let mut facp_address: u64 = 0 as u64;
        let facp_signature: u32 = 0x50434146 as u32; 
        let mut entries = (xsdt_entry as u64);
        

        for i in 0..total_entries {

            let facp_sig = ((((entries as *const u64).read()) as *const u32).read()) as u32;

            if facp_sig == facp_signature {
            	facp_address = (entries as *const u64).read();
            	//print_screen::print_screen(fb_ptr, stride, 0, help_cmd, 0x00FF00, "[+] FACP ka address mil gaya.");
            	break;
            }

            entries = entries.add(8);
                    	
        }

        let facp_address_u8 = facp_address as *const u8;
        let PM1a_CNT_BLK = ((facp_address_u8.add(64)) as *const u32).read();
        let pm1a_ioport: u16 = PM1a_CNT_BLK as u16;

        let val = (7 << 10) | 0x2000;
        asm!(
        	"out dx, ax",
        	in("dx") pm1a_ioport,
        	in("ax") val as u16
        );

        loop {
        	asm!("hlt");
        }

    }
	
}

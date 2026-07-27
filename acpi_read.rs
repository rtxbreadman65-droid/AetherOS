use asm;
use print;

pub unsafe fn acpi_read(fb_ptr: *mut u32, stride: usize, color: u32, rdsp: usize) {

    let rdsp_u8_const = rdsp as *const u8;
    let rdsp_revision = rdsp_u8_const.add(15);
    let rdsp_revision_read = rdsp_revision.read();

    if rdsp_revision_read == 2 {
    	print::print(fb_ptr, stride, 10, color, "[+] Your PC support ACPI 2.");
    }

    else {
    	print::print(fb_ptr, stride, 10, color, "[+] Your PC does not support ACPI 2.");
    	loop {
    		asm!("NOP");
    	}
    }
	
}

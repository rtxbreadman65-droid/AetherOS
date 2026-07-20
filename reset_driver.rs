use asm;

pub unsafe fn reset_driver(rdsp: usize) {

    let acpi_ptr = rdsp as *const u8;
    let ptr = acpi_ptr.add(24);

    let xsdt_header = ptr as *const u64;
    let xsdt_header_read = xsdt_header.read();
    let xsdt_u32 = xsdt_header_read as *const u32;
    
    let xsdt_lenght = xsdt_u32.add(1);
    let xsdt_lenght_read = xsdt_lenght.read();
    let total_pointer = (xsdt_lenght_read - 36) / 8;

    let xsdt_offset_jump = xsdt_u32 as *const u8;
    let jump = xsdt_offset_jump.add(36);

    let xsdt_u64 = jump as *const u64;

    let fadt_signature = 0x50434146;
    let mut fadt_address: u64 = 0;

    for fadt in 0..total_pointer {
        let table_address = xsdt_u64.add(fadt as usize);
    	let table_read = table_address.read();

        let signature = (table_read as *const u32).read();
    	
    	if signature == fadt_signature {
    		fadt_address = table_read;
    		break;
    	};
    	
    }

    let fadt_address_u8 = fadt_address as *const u8;
    let fadt_address_112 = fadt_address_u8.add(112);

    let fadt_address_u32 = fadt_address_112 as *const u32;
    let fadt_address_read = fadt_address_u32.read();

    if ((fadt_address_read >> 10) & 1) == 1 {}

    let fadt_address_120 = fadt_address_u8.add(120);
    let fadt_address_u64_read = (fadt_address_120 as *const u64).read();
    let target_address = fadt_address_u64_read;

    let fadt_address_128 = fadt_address_u8.add(128);
    let fadt_address_u8_read = (fadt_address_128 as *const u8).read();
    let magic_value = fadt_address_u8_read;

    let address_id = fadt_address_u8.add(116);
    let address_id_read = (address_id as *const u8).read();

    if address_id_read == 1 {

        asm!(
        	"out dx, al",
        	in("dx") target_address as u16,
        	in("al") 0x02 as u8
        );

        asm!(
        	"out dx, al",
        	in("dx") target_address as u16,
        	in("al") magic_value
        );
    	
    }

    else {
        let reset_key = target_address as *mut u8;
        let _ = reset_key.write_volatile(magic_value);
    }
      	
}

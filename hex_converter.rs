pub static HEX_CONVERT: [u8; 16] = *b"0123456789ABCDEF";

#[no_mangle]

pub fn hex_converter(mut address: u64, buf: &mut [u8; 16]) {

    unsafe {

        for i in (0..16).rev() {

            let last_4_bits = address & 0xF;
            let character = HEX_CONVERT[last_4_bits as usize];
            buf[i] = character;
            address >>= 4;
        	
        }
    	
    }
	
}

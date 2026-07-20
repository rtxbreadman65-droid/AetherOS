use font::FONT_BITMAPS;

pub fn print(fb: *mut u32, y: usize, stride: usize, color: u32, text: &str) {

    let text_len = text.len() * 8;

    let mut starting_with = (stride - text_len) / 2;
    let starting_height = y;

    unsafe {

        for c in text.bytes() {
        	let map = FONT_BITMAPS[c as usize];

            for row in 0..8 {
            	for col in 0..8 {

                    if (map[row] >> (7 - col)) & 1 == 1 {
                    	*fb.add((starting_height + row) * stride + (starting_with + col)) = color;
                    }
            		
            	}
            }

        starting_with += 8;
        	
        }
    	
    }
	
}

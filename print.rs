use font::FONT_BITMAPS;

pub fn print(fb: *mut u32, stride: usize, y_start: usize, color: u32, text: &str) {

    let mut starting_position = 0;

    unsafe {

        for c in text.bytes() {
        	let map = FONT_BITMAPS[c as usize];

            for row in 0..8 {
            	for col in 0..8 {

                    if (map[row] >> (7 - col)) & 1 == 1 {
                    	*fb.add((y_start + row) * stride + (starting_position + col)) = color;
                    }
            		
            	}
            }
        	starting_position += 5
        }
    	
    }
	
}

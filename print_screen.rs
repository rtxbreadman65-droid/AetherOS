use font::FONT_BITMAPS;

pub fn print_screen(fb: *mut u32, stride: usize, screen_with_x: usize, screen_height_y: usize, color: u32, text: &str) {

    unsafe {

        let mut starting_position = screen_with_x;

        for c in text.bytes() {
        
            if (c as usize) >= FONT_BITMAPS.len() {
            	continue;
            }

            let map = FONT_BITMAPS[c as usize];

            for row in 0..8 {
            	for col in 0..8 {

                    if (map[row] >> (7 - col)) & 1 == 1 {
                    	*fb.add((screen_height_y + row) * stride + (starting_position + col)) = color;
                    }
            	}
            }
            starting_position += 5;
	    }
    }
}

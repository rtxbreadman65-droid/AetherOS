pub fn patch_screen(fb_ptr: *mut u32, stride: usize, screen_with: usize, screen_height: usize, color: u32, column: usize) {

    let mut starting_position = screen_with;

    unsafe {

        for row in 0..8 {
        	for col in 0..column {

                *fb_ptr.add((screen_height + row) * stride + (starting_position + col)) = color;
        		
        	}
        }

        starting_position += 1;
    	
    }
	
}

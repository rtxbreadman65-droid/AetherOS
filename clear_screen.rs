pub fn clear_screen(fb: *mut u32, screen_with: usize, screen_height: usize, background_color: u32) {

    unsafe {

            let total_pixel = screen_with * screen_height;
            for i in 0..total_pixel {
                core::ptr::write(fb.add(i), background_color);
            	
        }
    	
    }
	
}

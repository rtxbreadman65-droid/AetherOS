const B: u32 = 0xFFD700;
const W: u32 = 0xFFFFFF; 

pub const LOGO_ROWS: [[u32; 8]; 8] = [
    [B, B, W, W, W, W, B, B], // 1 <-- Forehead
    [B, W, W, W, W, W, W, B], // 2
    [B, W, B, W, W, B, W, B], // 3 <-- Eyes
    [B, W, W, B, B, W, W, B], // 4 <-- Nose
    [B, B, W, W, W, W, B, B], // 5 <-- Jaw
    [B, B, W, B, B, W, B, B], // 6 <-- Teeth
    [B, B, B, W, W, B, B, B], // 7 <-- Chin
    [B, B, B, B, B, B, B, B], // 8
];

pub unsafe fn skull(fb: *mut u32, stride: usize, skull_size: usize, start_y: usize) {

    let start_x = (stride - skull_size) / 2;

    for y in 0..skull_size {
    	for x in 0..skull_size {

            let color = LOGO_ROWS[y / 40][x / 40];
            let offset = (start_y + y) * stride + (start_x + x);

            core::ptr::write(fb.add(offset), color);
    		
    	}
    }
	
}

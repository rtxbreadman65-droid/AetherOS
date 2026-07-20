const G: u32 = 0x00FF00;
const B: u32 = 0x000000;

pub const LOGO_ROWS: [[u32; 16]; 16] = [
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 1
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 2
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 3
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 4
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 5
    //   A       E       O       S
    [B,G,B,B,G,G,G,B,G,G,G,B,G,G,G,B], // 6
    [G,B,G,B,G,B,B,B,G,B,G,B,G,B,B,B], // 7
    [G,G,G,B,G,G,G,B,G,B,G,B,G,G,G,B], // 8
    [G,B,G,B,G,B,B,B,G,B,G,B,B,B,G,B], // 9
    [G,B,G,B,G,G,G,B,G,G,G,B,G,G,G,B], // 10
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 11
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 12
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 13
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 14
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 15
    [B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B], // 16
];

pub unsafe fn logo(fb: *mut u32, stride: usize, logo_size: usize, start_y: usize) {

    let start_x = (stride - logo_size) / 2;

    for y in 0..logo_size {
    	for x in 0..logo_size {

            let color = LOGO_ROWS[y / 20][x / 20];
            let offset = (start_y + y) * stride + (start_x + x);

            core::ptr::write(fb.add(offset), color);
    		
    	}
    }
	
}

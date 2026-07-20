use KEYBOARD_MAP::KEYBOARD_MAP;
use crate::apic_read;
use crate::print_screen;
use crate::GLOBALS;
use crate::push_all_registers;
use crate::pop_all_registers;
use crate::idt::help_cmd;
use crate::patch_screen;
use asm;

pub static mut current_position: usize = 30;
pub static mut current_line: usize = 0;

//#[link_section = ".text"]
pub static mut INPUT_BUFFER: [u8; 1056] = [0; 1056];
pub static mut INPUT_LEN: usize = 0;

const HEX_LOOKUP: [u8; 128] = {
    let mut map = [0; 128];
    map[b'0' as usize] = 0x0; map[b'1' as usize] = 0x1; map[b'2' as usize] = 0x2;
    map[b'3' as usize] = 0x3; map[b'4' as usize] = 0x4; map[b'5' as usize] = 0x5;
    map[b'6' as usize] = 0x6; map[b'7' as usize] = 0x7; map[b'8' as usize] = 0x8;
    map[b'9' as usize] = 0x9; map[b'a' as usize] = 0xA; map[b'b' as usize] = 0xB;
    map[b'c' as usize] = 0xC; map[b'd' as usize] = 0xD; map[b'e' as usize] = 0xE;
    map[b'f' as usize] = 0xF;
    map
};

#[no_mangle]
pub extern "C" fn keyboard_read() {

    unsafe {

        let keyboard_status_port: u16 = 0x64 as u16;
        let mut status: u8 = 0 as u8;

        asm!(
        	"in al, dx",
        	in("dx") keyboard_status_port,
        	out("al") status
        );

        let current_status = (status << 7);

        if current_status == 0b00000000 {
        	return
        }

        let data_port: u16 = 0x60 as u16;
        let mut scancode: u8 = 0 as u8;

        asm!(
        	"in al, dx",
        	in("dx") data_port, 
        	out("al") scancode
        );

        if scancode >= 0x80 {
        	return
        }

        if scancode < 0x80 {

            if scancode == 0xAA || scancode == 0xFA {
            	return;
            }

            if current_line > GLOBALS.GLOBAL_STRIDE || current_line > GLOBALS.GLOBAL_WITH || current_line > GLOBALS.GLOBAL_HEIGHT {
            	current_line = 0;
            }

            if help_cmd > GLOBALS.GLOBAL_STRIDE || help_cmd > GLOBALS.GLOBAL_WITH || help_cmd > GLOBALS.GLOBAL_HEIGHT {
            	help_cmd = 0;
            }

            if scancode == 0x1C {
                current_line += 40;
                help_cmd += 20;

                if help_cmd > 20 {
                	help_cmd += 20
                }

                if INPUT_BUFFER.len() > 0 {

                    asm!(
                        "call {0}",
                        in(reg) INPUT_BUFFER.as_ptr()
                    );

                    INPUT_BUFFER.fill(0);
                    INPUT_LEN = 0;

                }             

                print_screen::print_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, 0, current_line, 0x00FF00, "ROOT#");
            	current_position = 30;
            	return
            }

            if scancode == 0x0E {

                if current_position > GLOBALS.GLOBAL_STRIDE || current_position > GLOBALS.GLOBAL_WITH || current_position > GLOBALS.GLOBAL_HEIGHT {
                    current_position = 30;
                };

                if current_position >= 30 {
                
            	current_position -= 15;
            	patch_screen::patch_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, current_position, current_line, 0x000000, 8);

                if INPUT_LEN > 0 {
                	INPUT_LEN -= 1;
                	let idx = INPUT_LEN / 2;

                	INPUT_BUFFER[idx] &= 0xF0;
                	
                }

                }
            	
            }

            if let Some(ch) = KEYBOARD_MAP[scancode as usize] {

                let mut buffer = [0u8; 4];
                let ch_str = ch.encode_utf8(&mut buffer);

                let val = HEX_LOOKUP[ch as usize];

                if INPUT_LEN > 1000000 {
                	INPUT_LEN = 0;
                	return
                }
                
                if INPUT_LEN % 2 == 0 {
                    INPUT_BUFFER[INPUT_LEN / 2] = val;
                }           
        
                else {
                    INPUT_BUFFER[INPUT_LEN / 2] = (INPUT_BUFFER[INPUT_LEN / 2] << 4) | val;
                }

                INPUT_LEN += 1;

                asm!("out 0x80, al");
                asm!("out 0x80, al");

                if current_position > GLOBALS.GLOBAL_STRIDE || current_position > GLOBALS.GLOBAL_WITH || current_position > GLOBALS.GLOBAL_HEIGHT {
                	current_position = 30;
                }

                print_screen::print_screen(GLOBALS.GLOBAL_FB_PTR, GLOBALS.GLOBAL_STRIDE, current_position, current_line, 0x00FF00, ch_str);

                current_position += 15;
                let global_stride = GLOBALS.GLOBAL_STRIDE - 8;

                if current_position >= global_stride {
                	current_position = 30;
                }
                
                return

            }
    
        }

    }

}

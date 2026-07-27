use asm;

pub unsafe fn chip_status() -> u8 {

    let ec_cmd: u16 = 0x66 as u16;
    let mut status: u8 = 0 as u8;

    asm!(
    	"in al, dx",
    	in("dx") ec_cmd,
    	out("al") status
    );

    status
	
}

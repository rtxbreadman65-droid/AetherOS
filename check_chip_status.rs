use asm;

pub unsafe fn chip_status() -> u8 {

    let EC_cmd: u16 = 0x66;
    let mut chip_status: u8 = 0 as u8;

    asm!(
    	"in al, dx",
    	in("dx") EC_cmd,
    	out("al") chip_status
    );

    chip_status
	
}

use asm;

pub unsafe fn heatreader() -> u32 {

    let mut eax: u32 = 0 as u32;
    let mut edx: u32 = 0 as u32;
    let sensor_address: u16 = 0x19C as u16;

    asm!(
    
    	"rdmsr",
    	in("ecx") sensor_address,
    	out("eax") eax,
    	out("edx") edx,
    	options(nomem, nostack)
    	
    );

    let sensor_reading = (eax >> 16) & 0x7F;
    let heat = 100 - sensor_reading;

    heat
	
}

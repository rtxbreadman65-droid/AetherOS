use asm;

pub unsafe fn pc_speaker() {

    let pit_command_port: u16 = 0x43;
    let pit_channel_data: u16 = 0x42;
    let system_control_port: u16 = 0x61;

    asm!("out dx, al", in("dx") pit_command_port, in("al") 0xB6 as u8);

    let divisor: u16 = 4000;
    let divisor_u8 = divisor as u8;

    asm!("out dx, al", in("dx") pit_channel_data, in("al") divisor_u8);

    let divisor_bit_8 = (divisor >> 8) as u8;

    asm!("out dx, al", in("dx") pit_channel_data, in("al") divisor_bit_8);

    let mut current_status: u8 = 0;
    asm!("in al, dx", in("dx") system_control_port, out("al") current_status);

    let new_status = current_status | 3;
    asm!("out dx, al", in("dx") system_control_port, in("al") new_status);
	
}

pub unsafe fn ConvertStr(mut num: u64, buf: &mut [u8; 20]) -> &str {

    let mut i = 20;

    loop {

        i -= 1;
        buf[i] = b'0' + (num % 10) as u8;
        num /= 10;

        if num == 0 {
        	break;
        }
    	
    }

    core::str::from_utf8_unchecked(&buf[i..])
	
}

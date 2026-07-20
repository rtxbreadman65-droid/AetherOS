pub const KEYBOARD_MAP: [Option<char>; 128] = [
    // 0x00 - 0x0F
    None, None, Some('1'), Some('2'), Some('3'), Some('4'), Some('5'), Some('6'), 
    Some('7'), Some('8'), Some('9'), Some('0'), Some('-'), Some('='), None, None, 
    
    // 0x10 - 0x1F
    Some('q'), Some('w'), Some('e'), Some('r'), Some('t'), Some('y'), Some('u'), Some('i'), 
    Some('o'), Some('p'), Some('['), Some(']'), None, None, Some('a'), Some('s'),   
    
    // 0x20 - 0x2F
    Some('d'), Some('f'), Some('g'), Some('h'), Some('j'), Some('k'), Some('l'), Some(';'), 
    Some('\''), Some('`'), None, Some('\\'), Some('z'), Some('x'), Some('c'), Some('v'),  
    
    // 0x30 - 0x3F
    Some('b'), Some('n'), Some('m'), Some(','), Some('.'), Some('/'), None, None, 
    None, Some(' '), None, None, None, None, None, None, 

    // 0x40 - 0x4F (Padding baki keys ke liye taake pure 128 elements hon)
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    // 0x50 - 0x5F
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    // 0x60 - 0x6F
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    // 0x70 - 0x7F
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

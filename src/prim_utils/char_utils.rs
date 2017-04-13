use std::char;

pub const NUMBER_OF_CHARS: u32 = 0x10F800;

pub fn char_to_contiguous_range(c: char) -> u32 {
    if c <= '\u{D7FF}' {
        c as u32
    } else {
        c as u32 - 2048
    }
}

pub fn contiguous_range_to_char(i: u32) -> Option<char> {
    if i <= 0xD7FF {
        char::from_u32(i)
    } else if i < NUMBER_OF_CHARS {
        char::from_u32(i + 2048)
    } else {
        None
    }
}

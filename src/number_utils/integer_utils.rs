extern crate rugint;

use self::rugint::Integer;
use std::cmp::Ordering;

pub fn usize_bit_count() -> u32 {
    (0 as usize).count_zeros()
}

pub fn isize_bit_count() -> u32 {
    (0 as isize).count_zeros()
}

pub fn is_power_of_two(n: &Integer) -> bool {
    if n.sign() != Ordering::Greater {
        panic!("n must be positive. Invalid n: {}", n);
    }
    n.count_ones().unwrap() == 1
}

macro_rules! ceiling_log_2 {
    ($cl2: ident, $t: ty, $s: expr) => {
        pub fn $cl2(n: $t) -> u32 {
            if n < 1 {
                panic!("n must be positive. Invalid n: {}", n);
            }
            let bit_length = $s - n.leading_zeros();
            if n & (n - 1) == 0 {
                bit_length - 1
            } else {
                bit_length
            }
        }
    }
}

ceiling_log_2!(ceiling_log_2_u8, u8, 8);
ceiling_log_2!(ceiling_log_2_u16, u16, 16);
ceiling_log_2!(ceiling_log_2_u32, u32, 32);
ceiling_log_2!(ceiling_log_2_u64, u64, 64);
ceiling_log_2!(ceiling_log_2_usize, usize, usize_bit_count());
ceiling_log_2!(ceiling_log_2_i8, i8, 8);
ceiling_log_2!(ceiling_log_2_i16, i16, 16);
ceiling_log_2!(ceiling_log_2_i32, i32, 32);
ceiling_log_2!(ceiling_log_2_i64, i64, 64);
ceiling_log_2!(ceiling_log_2_isize, isize, isize_bit_count());

pub fn ceiling_log_2_integer(n: &Integer) -> u32 {
    if n.sign() != Ordering::Greater {
        panic!("n must be positive. Invalid n: {}", n);
    }
    let bit_length = n.significant_bits();
    if n.count_ones().unwrap() == 1 {
        bit_length - 1
    } else {
        bit_length
    }
}

macro_rules! bits_u {
    ($t: ty, $b: ident) => {
        pub fn $b(n: $t) -> Vec<bool> {
            let mut bits = Vec::new();
            let mut remaining = n;
            while remaining != 0 {
                bits.push(remaining & 1 != 0);
                remaining >>= 1;
            }
            bits
        }
    }
}

bits_u!(u8, bits_u8);
bits_u!(u16, bits_u16);
bits_u!(u32, bits_u32);
bits_u!(u64, bits_u64);
bits_u!(usize, bits_usize);

macro_rules! bits_i {
    ($t: ty, $b: ident) => {
        pub fn $b(n: $t) -> Vec<bool> {
            if n < 0 {
                panic!("n cannot be negative. Invalid n: {}", n);
            }
            let mut bits = Vec::new();
            let mut remaining = n;
            while remaining != 0 {
                bits.push(remaining & 1 != 0);
                remaining >>= 1;
            }
            bits
        }
    }
}

bits_i!(i8, bits_i8);
bits_i!(i16, bits_i16);
bits_i!(i32, bits_i32);
bits_i!(i64, bits_i64);
bits_i!(isize, bits_isize);

pub fn bits_integer(n: &Integer) -> Vec<bool> {
    match n.sign() {
        Ordering::Less => panic!("n cannot be negative. Invalid n: {}", n),
        Ordering::Equal => Vec::new(),
        Ordering::Greater => {
            let bit_length = n.significant_bits();
            let mut bits = Vec::with_capacity(bit_length as usize);
            for i in 0..bit_length {
                bits.push(n.get_bit(i));
            }
            bits
        }
    }
}

macro_rules! bits_padded_u {
    ($t: ty, $bp: ident) => {
        pub fn $bp(size: usize, n: $t) -> Vec<bool> {
            let mut bits = Vec::with_capacity(size);
            let mut remaining = n;
            for _ in 0..size {
                bits.push(remaining & 1 != 0);
                remaining >>= 1;
            }
            bits
        }
    }
}

bits_padded_u!(u8, bits_padded_u8);
bits_padded_u!(u16, bits_padded_u16);
bits_padded_u!(u32, bits_padded_u32);
bits_padded_u!(u64, bits_padded_u64);
bits_padded_u!(usize, bits_padded_usize);

macro_rules! bits_padded_i {
    ($t: ty, $bp: ident) => {
        pub fn $bp(size: usize, n: $t) -> Vec<bool> {
            if n < 0 {
                panic!("n cannot be negative. Invalid n: {}", n);
            }
            let mut bits = Vec::with_capacity(size);
            let mut remaining = n;
            for _ in 0..size {
                bits.push(remaining & 1 != 0);
                remaining >>= 1;
            }
            bits
        }
    }
}

bits_padded_i!(i8, bits_padded_i8);
bits_padded_i!(i16, bits_padded_i16);
bits_padded_i!(i32, bits_padded_i32);
bits_padded_i!(i64, bits_padded_i64);
bits_padded_i!(isize, bits_padded_isize);

pub fn bits_padded_integer(size: usize, n: &Integer) -> Vec<bool> {
    if n.sign() == Ordering::Less {
        panic!("n cannot be negative. Invalid n: {}", n);
    }
    let mut bits = Vec::with_capacity(size);
    for i in 0..(size as u32) {
        bits.push(n.get_bit(i));
    }
    bits
}

macro_rules! big_endian_bits_u {
    ($t: ty, $beb: ident, $size: expr) => {
        pub fn $beb(n: $t) -> Vec<bool> {
            let mut bits = Vec::new();
            if n == 0 {
                return bits;
            }
            let mut mask = 1 << ($size - n.leading_zeros() - 1);
            while mask != 0 {
                bits.push(n & mask != 0);
                mask >>= 1;
            }
            bits
        }
    }
}

big_endian_bits_u!(u8, big_endian_bits_u8, 8);
big_endian_bits_u!(u16, big_endian_bits_u16, 16);
big_endian_bits_u!(u32, big_endian_bits_u32, 32);
big_endian_bits_u!(u64, big_endian_bits_u64, 64);
big_endian_bits_u!(usize, big_endian_bits_usize, usize_bit_count());

macro_rules! big_endian_bits_i {
    ($t: ty, $beb: ident, $size: expr) => {
        pub fn $beb(n: $t) -> Vec<bool> {
            match n.signum() {
                -1 => panic!("n cannot be negative. Invalid n: {}", n),
                0 => Vec::new(),
                1 => {
                    let mut bits = Vec::new();
                    let mut mask = 1 << ($size - n.leading_zeros() - 1);
                    while mask != 0 {
                        bits.push(n & mask != 0);
                        mask >>= 1;
                    }
                    bits
                },
                _ => unreachable!()
            }
        }
    }
}

big_endian_bits_i!(i8, big_endian_bits_i8, 8);
big_endian_bits_i!(i16, big_endian_bits_i16, 16);
big_endian_bits_i!(i32, big_endian_bits_i32, 32);
big_endian_bits_i!(i64, big_endian_bits_i64, 64);
big_endian_bits_i!(isize, big_endian_bits_isize, isize_bit_count());

pub fn big_endian_bits_integer(n: &Integer) -> Vec<bool> {
    match n.sign() {
        Ordering::Less => panic!("n cannot be negative. Invalid n: {}", n),
        Ordering::Equal => Vec::new(),
        Ordering::Greater => {
            let bit_length = n.significant_bits();
            let mut bits = Vec::with_capacity(bit_length as usize);
            let mut i = bit_length - 1;
            loop {
                bits.push(n.get_bit(i));
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
            bits
        }
    }
}

macro_rules! big_endian_bits_padded_u {
    ($t: ty, $bebp: ident, $max_bits: expr) => {
        pub fn $bebp(size: usize, n: $t) -> Vec<bool> {
            let mut bits = Vec::new();
            if size == 0 {
                return bits;
            }
            let mut size = size;
            let max_bits = $max_bits;
            while size > max_bits {
                bits.push(false);
                size -= 1;
            }
            let mut mask = 1 << (size - 1);
            while mask != 0 {
                bits.push(n & mask != 0);
                mask >>= 1;
            }
            bits
        }
    }
}

big_endian_bits_padded_u!(u8, big_endian_bits_padded_u8, 8);
big_endian_bits_padded_u!(u16, big_endian_bits_padded_u16, 16);
big_endian_bits_padded_u!(u32, big_endian_bits_padded_u32, 32);
big_endian_bits_padded_u!(u64, big_endian_bits_padded_u64, 64);
big_endian_bits_padded_u!(usize,
                          big_endian_bits_padded_usize,
                          usize_bit_count() as usize);

macro_rules! big_endian_bits_padded_i {
    ($t: ty, $bebp: ident, $max_bits: expr) => {
        pub fn $bebp(size: usize, n: $t) -> Vec<bool> {
            if n < 0 {
                panic!("n cannot be negative. Invalid n: {}", n);
            }
            let mut bits = Vec::new();
            if size == 0 {
                return bits;
            }
            let mut size = size;
            let max_bits = $max_bits;
            while size > max_bits {
                bits.push(false);
                size -= 1;
            }
            let mut mask = 1 << (size - 1);
            while mask != 0 {
                bits.push(n & mask != 0);
                mask >>= 1;
            }
            bits
        }
    }
}

big_endian_bits_padded_i!(i8, big_endian_bits_padded_i8, 7);
big_endian_bits_padded_i!(i16, big_endian_bits_padded_i16, 15);
big_endian_bits_padded_i!(i32, big_endian_bits_padded_i32, 31);
big_endian_bits_padded_i!(i64, big_endian_bits_padded_i64, 63);
big_endian_bits_padded_i!(isize,
                          big_endian_bits_padded_isize,
                          (isize_bit_count() - 1) as usize);

pub fn big_endian_bits_padded_integer(size: usize, n: &Integer) -> Vec<bool> {
    if n.sign() == Ordering::Less {
        panic!("n cannot be negative. Invalid n: {}", n);
    }
    let mut bits = Vec::new();
    if size == 0 {
        return bits;
    }
    let mut i = size - 1;
    loop {
        bits.push(n.get_bit(i as u32));
        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }
    bits
}

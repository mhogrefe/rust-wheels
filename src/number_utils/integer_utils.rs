extern crate gmp;

use self::gmp::mpz::Mpz;

macro_rules! is_power_of_two {
    ($ipot: ident, $t: ty) => {
        pub fn $ipot(n: $t) -> bool {
            if n < 1 {
                panic!("n must be positive. Invalid n: {}", n);
            }
            n & (n - 1) == 0
        }
    }
}

is_power_of_two!(is_power_of_two_u8, u8);
is_power_of_two!(is_power_of_two_u16, u16);
is_power_of_two!(is_power_of_two_u32, u32);
is_power_of_two!(is_power_of_two_u64, u64);
is_power_of_two!(is_power_of_two_usize, usize);
is_power_of_two!(is_power_of_two_i8, i8);
is_power_of_two!(is_power_of_two_i16, i16);
is_power_of_two!(is_power_of_two_i32, i32);
is_power_of_two!(is_power_of_two_i64, i64);
is_power_of_two!(is_power_of_two_isize, isize);

pub fn is_power_of_two_mpz(n: &Mpz) -> bool {
    if n < &Mpz::from(1) {
        panic!("n must be positive. Invalid n: {}", n);
    }
    n.popcount() == 1
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
ceiling_log_2!(ceiling_log_2_i8, i8, 8);
ceiling_log_2!(ceiling_log_2_i16, i16, 16);
ceiling_log_2!(ceiling_log_2_i32, i32, 32);
ceiling_log_2!(ceiling_log_2_i64, i64, 64);

pub fn ceiling_log_2_mpz(n: &Mpz) -> u32 {
    if n < &Mpz::from(1) {
        panic!("n must be positive. Invalid n: {}", n);
    }
    let bit_length = n.size_in_base(2) as u32;
    if n.popcount() == 1 {
        bit_length - 1
    } else {
        bit_length
    }
}

pub fn bits_u8(n: u8) -> Vec<bool> {
    let mut bits = Vec::new();
    let mut remaining = n;
    while remaining != 0 {
        bits.push(remaining & 1 != 0);
        remaining >>= 1;
    }
    bits
}

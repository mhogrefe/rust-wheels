use std::char;

pub fn digit_to_char(i: u32) -> Option<char> {
    if i < 10 {
        char::from_u32(i + 48)
    } else if i < 36 {
        char::from_u32(i + 87)
    } else {
        None
    }
}

pub fn char_to_digit(c: char) -> Option<u32> {
    if c >= '0' && c <= '9' {
        Some(c as u32 - 48)
    } else if c >= 'a' && c <= 'z' {
        Some(c as u32 - 87)
    } else {
        None
    }
}

//pub fn digits_unsigned<T: PrimUnsignedInt>(radix: T, n: T) -> Vec<T> {
//    if radix < T::from_u8(2) {
//        panic!("radix must be at least 2. Invalid radix: {}", radix);
//    }
//    let zero = T::from_u8(0);
//    let one = T::from_u8(1);
//    let mut digits = Vec::new();
//    let log = ceiling_log_2_unsigned(radix);
//    let mut remaining = n;
//    if one << log == radix {
//        let mask = radix - one;
//        while remaining != zero {
//            digits.push(remaining & mask);
//            remaining >>= log;
//        }
//    } else {
//        while remaining != zero {
//            digits.push(remaining % radix);
//            remaining /= radix;
//        }
//    }
//    digits
//}

//pub fn digits_integer(radix: &Integer, n: &Integer) -> Vec<Integer> {
//    if *radix < 2 {
//        panic!("radix must be at least 2. Invalid radix: {}", radix);
//    }
//    let sign = n.sign();
//    if sign == Ordering::Less {
//        panic!("n cannot be negative. Invalid n: {}", n);
//    } else if sign == Ordering::Equal {
//        Vec::new()
//    } else if radix.natural_abs_ref().is_power_of_2() {
//        let log = ceiling_log_2_integer(radix);
//        let mut digits = Vec::new();
//        let length = n.significant_bits();
//        let mut digit = Integer::ZERO;
//        let mut i = 0;
//        let mut j = 0;
//        let mut mask = 1;
//        for x in n.natural_abs_ref().into_limbs_le().iter() {
//            loop {
//                if x & mask != 0 {
//                    digit.set_bit(j);
//                }
//                i += 1;
//                if i == length {
//                    break;
//                }
//                j += 1;
//                if j == log {
//                    let last_index = digits.len();
//                    digits.push(Integer::ZERO);
//                    mem::swap(&mut digits[last_index], &mut digit);
//                    j = 0;
//                }
//                if mask == 0 {
//                    break;
//                }
//                mask <<= 1;
//            }
//        }
//        if digit != 0 {
//            let last_index = digits.len();
//            digits.push(Integer::ZERO);
//            mem::swap(&mut digits[last_index], &mut digit);
//        }
//        digits
//    } else if *radix <= 36 {
//        big_endian_digits_integer(radix, n)
//            .into_iter()
//            .rev()
//            .collect()
//    } else {
//        let mut digits = Vec::new();
//        let mut remaining = n.clone();
//        while remaining != 0 {
//            let last_index = digits.len();
//            digits.push(radix.clone());
//            remaining.div_rem_in_place(&mut digits[last_index]);
//        }
//        digits
//    }
//}

//pub fn big_endian_digits_unsigned<T: PrimUnsignedInt>(radix: T, n: T) -> Vec<T> {
//    digits_unsigned(radix, n).into_iter().rev().collect()
//}

//pub fn big_endian_digits_integer(radix: &Integer, n: &Integer) -> Vec<Integer> {
//    if *radix < 2 {
//        panic!("radix must be at least 2. Invalid radix: {}", radix);
//    }
//    let sign = n.sign();
//    if sign == Ordering::Less {
//        panic!("n cannot be negative. Invalid n: {}", n);
//    } else if sign == Ordering::Equal {
//        Vec::new()
//    } else if *radix <= 36 {
//        n.to_string_radix(radix.to_i32().unwrap())
//            .chars()
//            .map(|c| Integer::from(char_to_digit(c).unwrap()))
//            .collect()
//    } else {
//        digits_integer(radix, n).into_iter().rev().collect()
//    }
//}

//pub fn from_digits(radix: &Integer, digits: &[Integer]) -> Integer {
//    if *radix < 2 {
//        panic!("radix must be at least 2. Invalid radix: {}", radix);
//    } else if digits.is_empty() {
//        Integer::ZERO
//    } else if radix.natural_abs_ref().is_power_of_2() {
//        let radix_log = ceiling_log_2_integer(radix);
//        let mut bits = Vec::new();
//        for d in digits {
//            if d.sign() == Ordering::Less {
//                panic!(
//                    "Each element of digits must be non-negative. Invalid digit: {} in {:?}",
//                    d, digits
//                );
//            } else if d >= radix {
//                panic!(
//                    "Each element of digits must be less than radix, which is {}. Invalid \
//                     digit: {} in {:?}",
//                    radix, d, digits
//                );
//            } else {
//                bits.append(&mut bits_padded_integer(radix_log as usize, d));
//            }
//        }
//        let mut result = Integer::ZERO;
//        result.assign_bits_unsigned(&bits);
//        result
//    } else if *radix <= 36 {
//        from_big_endian_digits(
//            radix,
//            &digits.iter().rev().cloned().collect::<Vec<Integer>>(),
//        )
//    } else {
//        let mut result = Integer::ZERO;
//        for d in digits.iter().rev() {
//            if d.sign() == Ordering::Less {
//                panic!(
//                    "Each element of digits must be non-negative. Invalid digit: {} in {:?}",
//                    d, digits
//                );
//            } else if d >= radix {
//                panic!(
//                    "Each element of digits must be less than radix, which is {}. Invalid \
//                     digit: {} in {:?}",
//                    radix, d, digits
//                );
//            } else {
//                result *= radix;
//                result += d.clone();
//            }
//        }
//        result
//    }
//}

//pub fn from_big_endian_digits(radix: &Integer, digits: &[Integer]) -> Integer {
//    if *radix < 2 {
//        panic!("radix must be at least 2. Invalid radix: {}", radix);
//    } else if digits.is_empty() {
//        Integer::ZERO
//    } else if radix.natural_abs_ref().is_power_of_2() {
//        let radix_log = ceiling_log_2_integer(radix);
//        let mut bits = Vec::new();
//        for d in digits.iter().rev() {
//            if d.sign() == Ordering::Less {
//                panic!(
//                    "Each element of digits must be non-negative. Invalid digit: {} in {:?}",
//                    d, digits
//                );
//            } else if d >= radix {
//                panic!(
//                    "Each element of digits must be less than radix, which is {}. Invalid \
//                     digit: {} in {:?}",
//                    radix, d, digits
//                );
//            } else {
//                bits.append(&mut bits_padded_integer(radix_log as usize, d));
//            }
//        }
//        let mut result = Integer::ZERO;
//        result.assign_bits_unsigned(&bits);
//        result
//    } else if *radix <= 36 {
//        let s: String = digits
//            .iter()
//            .map(|i| digit_to_char(i.to_u32().unwrap()).unwrap())
//            .collect();
//        Integer::from_str_radix(&s, radix.to_i32().unwrap()).unwrap()
//    } else {
//        let mut result = Integer::ZERO;
//        for d in digits {
//            if d.sign() == Ordering::Less {
//                panic!(
//                    "Each element of digits must be non-negative. Invalid digit: {} in {:?}",
//                    d, digits
//                );
//            } else if d >= radix {
//                panic!(
//                    "Each element of digits must be less than radix, which is {}. Invalid \
//                     digit: {} in {:?}",
//                    radix, d, digits
//                );
//            } else {
//                result *= radix;
//                result += d.clone();
//            }
//        }
//        result
//    }
//}

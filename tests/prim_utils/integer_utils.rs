use rust_wheels::prim_utils::integer_utils::*;

use malachite_nz::integer::Integer;
use malachite_base::num::NegativeOne;
use rust_wheels::prim_utils::traits::PrimUnsignedInt;
use std::str::FromStr;

//macro_rules! prim_fail {
//    (
//        $t: ty,
//        $ceiling_log_2_fail: ident,
//        $digits_fail_1: ident,
//        $digits_fail_2: ident,
//        $big_endian_digits_fail_1: ident,
//        $big_endian_digits_fail_2: ident,
//    ) => {
//        #[test]
//        #[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
//        fn $digits_fail_1() {
//            digits_unsigned::<$t>(1, 10);
//        }

//        #[test]
//        #[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
//        fn $digits_fail_2() {
//            digits_unsigned::<$t>(0, 10);
//        }

//        #[test]
//        #[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
//        fn $big_endian_digits_fail_1() {
//            big_endian_digits_unsigned::<$t>(1, 10);
//        }
//
//        #[test]
//        #[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
//        fn $big_endian_digits_fail_2() {
//            big_endian_digits_unsigned::<$t>(0, 10);
//        }
//    }
//}

//prim_fail!(
//    u8,
//    ceiling_log_2_u8_fail,
//    digits_u8_fail_1,
//    digits_u8_fail_2,
//    big_endian_digits_u8_fail_1,
//    big_endian_digits_u8_fail_2,
//);
//prim_fail!(
//    u16,
//    ceiling_log_2_u16_fail,
//    digits_u16_fail_1,
//    digits_u16_fail_2,
//    big_endian_digits_u16_fail_1,
//    big_endian_digits_u16_fail_2,
//);
//prim_fail!(
//    u32,
//    ceiling_log_2_u32_fail,
//    digits_u32_fail_1,
//    digits_u32_fail_2,
//    big_endian_digits_u32_fail_1,
//    big_endian_digits_u32_fail_2,
//);
//prim_fail!(
//    u64,
//    ceiling_log_2_u64_fail,
//    digits_u64_fail_1,
//    digits_u64_fail_2,
//    big_endian_digits_u64_fail_1,
//    big_endian_digits_u64_fail_2,
//);

fn bits_unsigned_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |n, out| assert_eq!(bits_unsigned(n), out);
    test(T::from_u8(0), vec![]);
    test(T::from_u8(1), vec![true]);
    test(T::from_u8(6), vec![false, true, true]);
    test(
        T::from_u8(105),
        vec![true, false, false, true, false, true, true],
    );
    test(T::max_value(), max_bits);
}

#[test]
fn test_bits_unsigned() {
    bits_unsigned_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    bits_unsigned_helper::<u16>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    bits_unsigned_helper::<u32>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true,
    ]);
    bits_unsigned_helper::<u64>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true,
    ]);
}

#[test]
fn test_bits_integer() {
    let test = |n, out| assert_eq!(bits_integer(&Integer::from_str(n).unwrap()), out);
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![false, true, true]);
    test("105", vec![true, false, false, true, false, true, true]);
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -5")]
fn bits_integer_fail() {
    bits_integer(&Integer::from(-5));
}

fn big_endian_bits_unsigned_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |n, out| assert_eq!(big_endian_bits_unsigned(n), out);
    test(T::from_u8(0), vec![]);
    test(T::from_u8(1), vec![true]);
    test(T::from_u8(6), vec![true, true, false]);
    test(
        T::from_u8(105),
        vec![true, true, false, true, false, false, true],
    );
    test(T::max_value(), max_bits);
}

#[test]
fn big_endian_test_bits_unsigned() {
    big_endian_bits_unsigned_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    big_endian_bits_unsigned_helper::<u16>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    big_endian_bits_unsigned_helper::<u32>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true,
    ]);
    big_endian_bits_unsigned_helper::<u64>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true,
    ]);
}

#[test]
fn big_endian_bits_integer_helper() {
    let test = |n, out| assert_eq!(big_endian_bits_integer(&Integer::from_str(n).unwrap()), out);
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![true, true, false]);
    test("105", vec![true, true, false, true, false, false, true]);
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -5")]
fn big_endian_bits_integer_fail() {
    big_endian_bits_integer(&Integer::from(-5));
}

//fn from_big_endian_bits_helper(bits: &[bool], out: &str) {
//    assert_eq!(format!("{}", from_big_endian_bits(&bits)), out);
//}
//
//#[test]
//fn test_from_big_endian_bits() {
//    from_big_endian_bits_helper(&[], "0");
//    from_big_endian_bits_helper(&[false, false], "0");
//    from_big_endian_bits_helper(&[false, true], "1");
//    from_big_endian_bits_helper(&[false, false, false, false, false, true, true, false], "6");
//    from_big_endian_bits_helper(&[true, true, false, true, false, false, true], "105");
//}

#[test]
fn test_digit_to_char() {
    let test = |i, out| assert_eq!(digit_to_char(i), out);
    test(0, Some('0'));
    test(6, Some('6'));
    test(10, Some('a'));
    test(20, Some('k'));
    test(35, Some('z'));
    test(36, None);
}

#[test]
fn test_char_to_digit() {
    let test = |c, out| assert_eq!(char_to_digit(c), out);
    test('0', Some(0));
    test('6', Some(6));
    test('a', Some(10));
    test('k', Some(20));
    test('z', Some(35));
    test(' ', None);
    test('A', None);
}

//fn digits_unsigned_helper<T: PrimUnsignedInt>(max_digit: &str) {
//    let test = |radix, n, out| assert_eq!(format!("{:?}", digits_unsigned(radix, n)), out);
//    test(T::from_u8(2), T::from_u8(0), "[]");
//    test(T::from_u8(3), T::from_u8(0), "[]");
//    test(T::from_u8(8), T::from_u8(0), "[]");
//    test(T::from_u8(10), T::from_u8(0), "[]");
//    test(T::from_u8(12), T::from_u8(0), "[]");
//    test(T::from_u8(57), T::from_u8(0), "[]");
//    test(T::from_u8(2), T::from_u8(1), "[1]");
//    test(T::from_u8(3), T::from_u8(1), "[1]");
//    test(T::from_u8(8), T::from_u8(1), "[1]");
//    test(T::from_u8(10), T::from_u8(1), "[1]");
//    test(T::from_u8(12), T::from_u8(1), "[1]");
//    test(T::from_u8(57), T::from_u8(1), "[1]");
//    test(T::from_u8(2), T::from_u8(10), "[0, 1, 0, 1]");
//    test(T::from_u8(3), T::from_u8(10), "[1, 0, 1]");
//    test(T::from_u8(8), T::from_u8(10), "[2, 1]");
//    test(T::from_u8(10), T::from_u8(10), "[0, 1]");
//    test(T::from_u8(12), T::from_u8(10), "[10]");
//    test(T::from_u8(57), T::from_u8(10), "[10]");
//    test(T::from_u8(2), T::from_u8(107), "[1, 1, 0, 1, 0, 1, 1]");
//    test(T::from_u8(3), T::from_u8(107), "[2, 2, 2, 0, 1]");
//    test(T::from_u8(8), T::from_u8(107), "[3, 5, 1]");
//    test(T::from_u8(10), T::from_u8(107), "[7, 0, 1]");
//    test(T::from_u8(12), T::from_u8(107), "[11, 8]");
//    test(T::from_u8(57), T::from_u8(107), "[50, 1]");
//    test(T::max_value(), T::from_u8(0), "[]");
//    test(T::max_value(), T::from_u8(107), "[107]");
//    test(T::max_value(), T::max_value() - T::from_u8(1), max_digit);
//    test(T::max_value(), T::max_value(), "[0, 1]");
//}
//
//#[test]
//fn test_digits_unsigned() {
//    digits_unsigned_helper::<u8>("[254]");
//    digits_unsigned_helper::<u16>("[65534]");
//    digits_unsigned_helper::<u32>("[4294967294]");
//    digits_unsigned_helper::<u64>("[18446744073709551614]");
//}

//#[test]
//fn test_digits_integer() {
//    let test = |radix, n, out| {
//        assert_eq!(
//            format!(
//                "{:?}",
//                digits_integer(
//                    &Integer::from_str(radix).unwrap(),
//                    &Integer::from_str(n).unwrap(),
//                )
//            ),
//            out
//        )
//    };
//    test("2", "0", "[]");
//    test("3", "0", "[]");
//    test("8", "0", "[]");
//    test("10", "0", "[]");
//    test("12", "0", "[]");
//    test("57", "0", "[]");
//    test("2", "1", "[1]");
//    test("3", "1", "[1]");
//    test("8", "1", "[1]");
//    test("10", "1", "[1]");
//    test("12", "1", "[1]");
//    test("57", "1", "[1]");
//    test("2", "10", "[0, 1, 0, 1]");
//    test("3", "10", "[1, 0, 1]");
//    test("8", "10", "[2, 1]");
//    test("10", "10", "[0, 1]");
//    test("12", "10", "[10]");
//    test("57", "10", "[10]");
//    test("2", "187", "[1, 1, 0, 1, 1, 1, 0, 1]");
//    test("3", "187", "[1, 2, 2, 0, 2]");
//    test("8", "187", "[3, 7, 2]");
//    test("10", "187", "[7, 8, 1]");
//    test("12", "187", "[7, 3, 1]");
//    test("57", "187", "[16, 3]");
//}
//
//fn digits_integer_fail_helper(radix: &str, n: &str) {
//    digits_integer(
//        &Integer::from_str(radix).unwrap(),
//        &Integer::from_str(n).unwrap(),
//    );
//}
//
//#[test]
//#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
//fn digits_integer_fail_1() {
//    digits_integer_fail_helper("1", "10");
//}
//
//#[test]
//#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
//fn digits_integer_fail_2() {
//    digits_integer_fail_helper("0", "10");
//}
//
//#[test]
//#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
//fn digits_integer_fail_3() {
//    digits_integer_fail_helper("2", "-1");
//}
//
//#[test]
//#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
//fn digits_integer_fail_4() {
//    digits_integer_fail_helper("0", "-1");
//}

//fn big_endian_digits_unsigned_helper<T: PrimUnsignedInt>(max_digit: &str) {
//    let test =
//        |radix, n, out| assert_eq!(format!("{:?}", big_endian_digits_unsigned(radix, n)), out);
//    test(T::from_u8(2), T::from_u8(0), "[]");
//    test(T::from_u8(3), T::from_u8(0), "[]");
//    test(T::from_u8(8), T::from_u8(0), "[]");
//    test(T::from_u8(10), T::from_u8(0), "[]");
//    test(T::from_u8(12), T::from_u8(0), "[]");
//    test(T::from_u8(57), T::from_u8(0), "[]");
//    test(T::from_u8(2), T::from_u8(1), "[1]");
//    test(T::from_u8(3), T::from_u8(1), "[1]");
//    test(T::from_u8(8), T::from_u8(1), "[1]");
//    test(T::from_u8(10), T::from_u8(1), "[1]");
//    test(T::from_u8(12), T::from_u8(1), "[1]");
//    test(T::from_u8(57), T::from_u8(1), "[1]");
//    test(T::from_u8(2), T::from_u8(10), "[1, 0, 1, 0]");
//    test(T::from_u8(3), T::from_u8(10), "[1, 0, 1]");
//    test(T::from_u8(8), T::from_u8(10), "[1, 2]");
//    test(T::from_u8(10), T::from_u8(10), "[1, 0]");
//    test(T::from_u8(12), T::from_u8(10), "[10]");
//    test(T::from_u8(57), T::from_u8(10), "[10]");
//    test(T::from_u8(2), T::from_u8(107), "[1, 1, 0, 1, 0, 1, 1]");
//    test(T::from_u8(3), T::from_u8(107), "[1, 0, 2, 2, 2]");
//    test(T::from_u8(8), T::from_u8(107), "[1, 5, 3]");
//    test(T::from_u8(10), T::from_u8(107), "[1, 0, 7]");
//    test(T::from_u8(12), T::from_u8(107), "[8, 11]");
//    test(T::from_u8(57), T::from_u8(107), "[1, 50]");
//    test(T::max_value(), T::from_u8(0), "[]");
//    test(T::max_value(), T::from_u8(107), "[107]");
//    test(T::max_value(), T::max_value() - T::from_u8(1), max_digit);
//    test(T::max_value(), T::max_value(), "[1, 0]");
//}
//
//#[test]
//fn test_big_endian_digits_unsigned() {
//    big_endian_digits_unsigned_helper::<u8>("[254]");
//    big_endian_digits_unsigned_helper::<u16>("[65534]");
//    big_endian_digits_unsigned_helper::<u32>("[4294967294]");
//    big_endian_digits_unsigned_helper::<u64>("[18446744073709551614]");
//}

//#[test]
//fn test_big_endian_digits_integer() {
//    let test = |radix, n, out| {
//        assert_eq!(
//            format!(
//                "{:?}",
//                big_endian_digits_integer(
//                    &Integer::from_str(radix).unwrap(),
//                    &Integer::from_str(n).unwrap(),
//                )
//            ),
//            out
//        )
//    };
//    test("2", "0", "[]");
//    test("3", "0", "[]");
//    test("8", "0", "[]");
//    test("10", "0", "[]");
//    test("12", "0", "[]");
//    test("57", "0", "[]");
//    test("2", "1", "[1]");
//    test("3", "1", "[1]");
//    test("8", "1", "[1]");
//    test("10", "1", "[1]");
//    test("12", "1", "[1]");
//    test("57", "1", "[1]");
//    test("2", "10", "[1, 0, 1, 0]");
//    test("3", "10", "[1, 0, 1]");
//    test("8", "10", "[1, 2]");
//    test("10", "10", "[1, 0]");
//    test("12", "10", "[10]");
//    test("57", "10", "[10]");
//    test("2", "187", "[1, 0, 1, 1, 1, 0, 1, 1]");
//    test("3", "187", "[2, 0, 2, 2, 1]");
//    test("8", "187", "[2, 7, 3]");
//    test("10", "187", "[1, 8, 7]");
//    test("12", "187", "[1, 3, 7]");
//    test("57", "187", "[3, 16]");
//}

//#[test]
//fn test_from_digits() {
//    let test = |radix, digits, out| {
//        assert_eq!(
//            format!(
//                "{:?}",
//                from_digits(
//                    &Integer::from_str(radix).unwrap(),
//                    &parse_vec(digits).unwrap(),
//                )
//            ),
//            out
//        )
//    };
//    test("2", "[]", "0");
//    test("3", "[]", "0");
//    test("8", "[]", "0");
//    test("10", "[]", "0");
//    test("12", "[]", "0");
//    test("57", "[]", "0");
//    test("2", "[1]", "1");
//    test("3", "[1]", "1");
//    test("8", "[1]", "1");
//    test("10", "[1]", "1");
//    test("12", "[1]", "1");
//    test("57", "[1]", "1");
//    test("2", "[0, 1, 0, 1]", "10");
//    test("3", "[1, 0, 1]", "10");
//    test("8", "[2, 1]", "10");
//    test("10", "[0, 1]", "10");
//    test("12", "[10]", "10");
//    test("57", "[10]", "10");
//    test("2", "[1, 1, 0, 1, 1, 1, 0, 1]", "187");
//    test("3", "[1, 2, 2, 0, 2]", "187");
//    test("8", "[3, 7, 2]", "187");
//    test("10", "[7, 8, 1]", "187");
//    test("12", "[7, 3, 1]", "187");
//    test("57", "[16, 3]", "187");
//}
//
//fn from_digits_fail_helper(radix: &str, digits: &str) {
//    from_digits(
//        &Integer::from_str(radix).unwrap(),
//        &parse_vec(digits).unwrap(),
//    );
//}
//
//#[test]
//#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
//fn from_digits_fail_1() {
//    from_digits_fail_helper("1", "[1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
//fn from_digits_fail_2() {
//    from_digits_fail_helper("0", "[1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "radix must be at least 2. Invalid radix: -1")]
//fn from_digits_fail_3() {
//    from_digits_fail_helper("-1", "[1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "Each element of digits must be non-negative. Invalid digit: -1 in \
//                           [-1, 0, 1]")]
//fn from_digits_fail_4() {
//    from_digits_fail_helper("2", "[-1, 0, 1]");
//}
//
//#[test]
//#[should_panic(expected = "Each element of digits must be less than radix, which is 2. Invalid \
//                           digit: 2 in [1, 0, 2]")]
//fn from_digits_fail_5() {
//    from_digits_fail_helper("2", "[1, 0, 2]");
//}
//
//#[test]
//#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
//fn from_digits_fail_6() {
//    from_digits_fail_helper("10", "[-1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "Result::unwrap()` on an `Err` value: ParseIntegerError { kind: \
//                           InvalidDigit }")]
//fn from_digits_fail_7() {
//    from_digits_fail_helper("10", "[1, 2, 10]");
//}
//
//#[test]
//#[should_panic(expected = "Each element of digits must be non-negative. Invalid digit: -1 in \
//                           [-1, 2, 3]")]
//fn from_digits_fail_8() {
//    from_digits_fail_helper("100", "[-1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "Each element of digits must be less than radix, which is 100. Invalid \
//                           digit: 100 in [1, 2, 100]")]
//fn from_digits_fail_9() {
//    from_digits_fail_helper("100", "[1, 2, 100]");
//}
//
//#[test]
//fn test_from_big_endian_digits() {
//    let test = |radix, digits, out| {
//        assert_eq!(
//            format!(
//                "{:?}",
//                from_big_endian_digits(
//                    &Integer::from_str(radix).unwrap(),
//                    &parse_vec(digits).unwrap(),
//                )
//            ),
//            out
//        )
//    };
//    test("2", "[]", "0");
//    test("3", "[]", "0");
//    test("8", "[]", "0");
//    test("10", "[]", "0");
//    test("12", "[]", "0");
//    test("57", "[]", "0");
//    test("2", "[1]", "1");
//    test("3", "[1]", "1");
//    test("8", "[1]", "1");
//    test("10", "[1]", "1");
//    test("12", "[1]", "1");
//    test("57", "[1]", "1");
//    test("2", "[1, 0, 1, 0]", "10");
//    test("3", "[1, 0, 1]", "10");
//    test("8", "[1, 2]", "10");
//    test("10", "[1, 0]", "10");
//    test("12", "[10]", "10");
//    test("57", "[10]", "10");
//    test("2", "[1, 0, 1, 1, 1, 0, 1, 1]", "187");
//    test("3", "[2, 0, 2, 2, 1]", "187");
//    test("8", "[2, 7, 3]", "187");
//    test("10", "[1, 8, 7]", "187");
//    test("12", "[1, 3, 7]", "187");
//    test("57", "[3, 16]", "187");
//}
//
//fn from_big_endian_digits_fail_helper(radix: &str, digits: &str) {
//    from_big_endian_digits(
//        &Integer::from_str(radix).unwrap(),
//        &parse_vec(digits).unwrap(),
//    );
//}
//
//#[test]
//#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
//fn from_big_endian_digits_fail_1() {
//    from_big_endian_digits_fail_helper("1", "[1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
//fn from_big_endian_digits_fail_2() {
//    from_big_endian_digits_fail_helper("0", "[1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "radix must be at least 2. Invalid radix: -1")]
//fn from_big_endian_digits_fail_3() {
//    from_big_endian_digits_fail_helper("-1", "[1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "Each element of digits must be non-negative. Invalid digit: -1 in \
//                           [-1, 0, 1]")]
//fn from_big_endian_digits_fail_4() {
//    from_big_endian_digits_fail_helper("2", "[-1, 0, 1]");
//}
//
//#[test]
//#[should_panic(expected = "Each element of digits must be less than radix, which is 2. Invalid \
//                           digit: 2 in [1, 0, 2]")]
//fn from_big_endian_digits_fail_5() {
//    from_big_endian_digits_fail_helper("2", "[1, 0, 2]");
//}
//
//#[test]
//#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
//fn from_big_endian_digits_fail_6() {
//    from_big_endian_digits_fail_helper("10", "[-1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "Result::unwrap()` on an `Err` value: ParseIntegerError { kind: \
//                           InvalidDigit }")]
//fn from_big_endian_digits_fail_7() {
//    from_big_endian_digits_fail_helper("10", "[1, 2, 10]");
//}
//
//#[test]
//#[should_panic(expected = "Each element of digits must be non-negative. Invalid digit: -1 in \
//                           [-1, 2, 3]")]
//fn from_big_endian_digits_fail_8() {
//    from_big_endian_digits_fail_helper("100", "[-1, 2, 3]");
//}
//
//#[test]
//#[should_panic(expected = "Each element of digits must be less than radix, which is 100. Invalid \
//                           digit: 100 in [1, 2, 100]")]
//fn from_big_endian_digits_fail_9() {
//    from_big_endian_digits_fail_helper("100", "[1, 2, 100]");
//}

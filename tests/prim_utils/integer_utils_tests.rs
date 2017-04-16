use gmp_to_flint_adaptor_lib::integer::Integer;
use rust_wheels_lib::prim_utils::integer_utils::*;
use rust_wheels_lib::prim_utils::traits::*;
use std::str::FromStr;

#[test]
fn test_is_power_of_two() {
    let test = |n, out| assert_eq!(is_power_of_two(&Integer::from_str(n).unwrap()), out);
    test("1", true);
    test("2", true);
    test("4", true);
    test("8", true);
    test("16", true);
    test("3", false);
    test("13", false);
}

fn is_power_of_two_fail_helper(n: &str) {
    is_power_of_two(&Integer::from_str(n).unwrap());
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn is_power_of_two_fail_1() {
    is_power_of_two_fail_helper("0");
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: -5")]
fn is_power_of_two_fail_2() {
    is_power_of_two_fail_helper("-5");
}

fn ceiling_log_2_u_helper<T: PrimUnsignedInt>() {
    let test = |n, out| assert_eq!(ceiling_log_2_u(n), out);
    test(T::from_u8(1), 0);
    test(T::from_u8(2), 1);
    test(T::from_u8(3), 2);
    test(T::from_u8(4), 2);
    test(T::from_u8(5), 3);
    test(T::from_u8(6), 3);
    test(T::from_u8(7), 3);
    test(T::from_u8(8), 3);
    test(T::from_u8(9), 4);
    test(T::from_u8(100), 7);
    test(T::max_value(), T::bit_count());
}

#[test]
fn test_ceiling_log_2_u() {
    ceiling_log_2_u_helper::<u8>();
    ceiling_log_2_u_helper::<u16>();
    ceiling_log_2_u_helper::<u32>();
    ceiling_log_2_u_helper::<u64>();
    ceiling_log_2_u_helper::<usize>();
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn test_ceiling_log_2_u8_fail() {
    ceiling_log_2_u(0u8);
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn test_ceiling_log_2_u16_fail() {
    ceiling_log_2_u(0u16);
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn test_ceiling_log_2_u32_fail() {
    ceiling_log_2_u(0u32);
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn test_ceiling_log_2_u64_fail() {
    ceiling_log_2_u(0u64);
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn test_ceiling_log_2_usize_fail() {
    ceiling_log_2_u(0usize);
}

#[test]
fn test_ceiling_log_2_integer() {
    let test = |n, out| assert_eq!(ceiling_log_2_integer(&Integer::from_str(n).unwrap()), out);
    test("1", 0);
    test("2", 1);
    test("3", 2);
    test("4", 2);
    test("5", 3);
    test("6", 3);
    test("7", 3);
    test("8", 3);
    test("9", 4);
    test("100", 7);
}

fn ceiling_log_2_integer_fail_helper(n: &str) {
    ceiling_log_2_integer(&Integer::from_str(n).unwrap());
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn ceiling_log_2_integer_fail_1() {
    ceiling_log_2_integer_fail_helper("0");
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: -5")]
fn ceiling_log_2_integer_fail_2() {
    ceiling_log_2_integer_fail_helper("-5");
}

fn bits_u_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |n, out| assert_eq!(bits_u(n), out);
    test(T::from_u8(0), vec![]);
    test(T::from_u8(1), vec![true]);
    test(T::from_u8(6), vec![false, true, true]);
    test(T::from_u8(105),
         vec![true, false, false, true, false, true, true]);
    test(T::max_value(), max_bits);
}

#[test]
fn test_bits_u() {
    bits_u_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    bits_u_helper::<u16>(vec![true, true, true, true, true, true, true, true, true, true, true,
                              true, true, true, true, true]);
    bits_u_helper::<u32>(vec![true, true, true, true, true, true, true, true, true, true, true,
                              true, true, true, true, true, true, true, true, true, true, true,
                              true, true, true, true, true, true, true, true, true, true]);
    bits_u_helper::<u64>(vec![true, true, true, true, true, true, true, true, true, true, true,
                              true, true, true, true, true, true, true, true, true, true, true,
                              true, true, true, true, true, true, true, true, true, true, true,
                              true, true, true, true, true, true, true, true, true, true, true,
                              true, true, true, true, true, true, true, true, true, true, true,
                              true, true, true, true, true, true, true, true, true]);
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

fn bits_padded_u_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |size, n, out| assert_eq!(bits_padded_u(size, n), out);
    test(8,
         T::from_u8(0),
         vec![false, false, false, false, false, false, false, false]);
    test(8,
         T::from_u8(1),
         vec![true, false, false, false, false, false, false, false]);
    test(8,
         T::from_u8(6),
         vec![false, true, true, false, false, false, false, false]);
    test(8,
         T::from_u8(105),
         vec![true, false, false, true, false, true, true, false]);
    test(2, T::from_u8(104), vec![false, false]);
    test(2, T::from_u8(105), vec![true, false]);
    test(1, T::from_u8(104), vec![false]);
    test(1, T::from_u8(105), vec![true]);
    test(0, T::from_u8(104), vec![]);
    test(100,
         T::from_u8(105),
         vec![true, false, false, true, false, true, true, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false]);
    test(T::bit_count() as usize, T::max_value(), max_bits);
}

#[test]
fn test_bits_padded_u() {
    bits_padded_u_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    bits_padded_u_helper::<u16>(vec![true, true, true, true, true, true, true, true, true, true,
                                     true, true, true, true, true, true]);
    bits_padded_u_helper::<u32>(vec![true, true, true, true, true, true, true, true, true, true,
                                     true, true, true, true, true, true, true, true, true, true,
                                     true, true, true, true, true, true, true, true, true, true,
                                     true, true]);
    bits_padded_u_helper::<u64>(vec![true, true, true, true, true, true, true, true, true, true,
                                     true, true, true, true, true, true, true, true, true, true,
                                     true, true, true, true, true, true, true, true, true, true,
                                     true, true, true, true, true, true, true, true, true, true,
                                     true, true, true, true, true, true, true, true, true, true,
                                     true, true, true, true, true, true, true, true, true, true,
                                     true, true, true, true]);
}

#[test]
fn test_bits_padded_integer() {
    let test = |size, n, out| {
        assert_eq!(bits_padded_integer(size, &Integer::from_str(n).unwrap()),
                   out)
    };
    test(8,
         "0",
         vec![false, false, false, false, false, false, false, false]);
    test(8,
         "1",
         vec![true, false, false, false, false, false, false, false]);
    test(8,
         "6",
         vec![false, true, true, false, false, false, false, false]);
    test(8,
         "105",
         vec![true, false, false, true, false, true, true, false]);
    test(2, "104", vec![false, false]);
    test(2, "105", vec![true, false]);
    test(1, "104", vec![false]);
    test(1, "105", vec![true]);
    test(0, "104", vec![]);
    test(100,
         "105",
         vec![true, false, false, true, false, true, true, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false]);
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
fn bits_padded_integer_fail() {
    bits_padded_integer(8, &Integer::from(-1));
}

fn big_endian_bits_u_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |n, out| assert_eq!(big_endian_bits_u(n), out);
    test(T::from_u8(0), vec![]);
    test(T::from_u8(1), vec![true]);
    test(T::from_u8(6), vec![true, true, false]);
    test(T::from_u8(105),
         vec![true, true, false, true, false, false, true]);
    test(T::max_value(), max_bits);
}

#[test]
fn big_endian_test_bits_u() {
    big_endian_bits_u_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    big_endian_bits_u_helper::<u16>(vec![true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true, true, true]);
    big_endian_bits_u_helper::<u32>(vec![true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true]);
    big_endian_bits_u_helper::<u64>(vec![true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true, true, true, true, true,
                                         true, true, true, true, true, true, true, true, true,
                                         true]);
}

#[test]
fn big_endian_bits_integer_helper() {
    let test = |n, out| assert_eq!(big_endian_bits_integer(&Integer::from_str(n).unwrap()), out);
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![true, true, false]);
    test("105", vec![true, true, false, true, false, false, true]);
}

fn big_endian_bits_padded_u_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |size, n, out| assert_eq!(big_endian_bits_padded_u(size, n), out);
    test(8,
         T::from_u8(0),
         vec![false, false, false, false, false, false, false, false]);
    test(8,
         T::from_u8(1),
         vec![false, false, false, false, false, false, false, true]);
    test(8,
         T::from_u8(6),
         vec![false, false, false, false, false, true, true, false]);
    test(8,
         T::from_u8(105),
         vec![false, true, true, false, true, false, false, true]);
    test(2, T::from_u8(104), vec![false, false]);
    test(2, T::from_u8(105), vec![false, true]);
    test(1, T::from_u8(104), vec![false]);
    test(1, T::from_u8(105), vec![true]);
    test(0, T::from_u8(104), vec![]);
    test(100,
         T::from_u8(105),
         vec![false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, true, true, false,
              true, false, false, true]);
    test(T::bit_count() as usize, T::max_value(), max_bits);
}

#[test]
fn test_big_endian_bits_padded_u() {
    big_endian_bits_padded_u_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    big_endian_bits_padded_u_helper::<u16>(vec![true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true]);
    big_endian_bits_padded_u_helper::<u32>(vec![true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true]);
    big_endian_bits_padded_u_helper::<u64>(vec![true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true,
                                                true, true, true, true, true, true, true, true]);
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -5")]
fn big_endian_bits_integer_fail() {
    big_endian_bits_integer(&Integer::from(-5));
}

fn big_endian_bits_padded_integer_helper(size: usize, n: &str, out: Vec<bool>) {
    assert_eq!(big_endian_bits_padded_integer(size, &Integer::from_str(n).unwrap()),
               out);
}

#[test]
fn test_big_endian_bits_padded_integer() {
    big_endian_bits_padded_integer_helper(8,
                                          "0",
                                          vec![false, false, false, false, false, false, false,
                                               false]);
    big_endian_bits_padded_integer_helper(8,
                                          "1",
                                          vec![false, false, false, false, false, false, false,
                                               true]);
    big_endian_bits_padded_integer_helper(8,
                                          "6",
                                          vec![false, false, false, false, false, true, true,
                                               false]);
    big_endian_bits_padded_integer_helper(8,
                                          "105",
                                          vec![false, true, true, false, true, false, false, true]);
    big_endian_bits_padded_integer_helper(2, "104", vec![false, false]);
    big_endian_bits_padded_integer_helper(2, "105", vec![false, true]);
    big_endian_bits_padded_integer_helper(1, "104", vec![false]);
    big_endian_bits_padded_integer_helper(1, "105", vec![true]);
    big_endian_bits_padded_integer_helper(0, "104", vec![]);
    big_endian_bits_padded_integer_helper(100,
                                          "105",
                                          vec![false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false, false, false, false,
                                               false, false, true, true, false, true, false,
                                               false, true]);
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
fn big_endian_bits_padded_integer_fail() {
    big_endian_bits_padded_integer(8, &Integer::from(-1));
}

fn from_big_endian_bits_helper(bits: Vec<bool>, out: &str) {
    assert_eq!(format!("{}", from_big_endian_bits(&bits[..])), out);
}

#[test]
fn test_from_big_endian_bits() {
    from_big_endian_bits_helper(vec![], "0");
    from_big_endian_bits_helper(vec![false, false], "0");
    from_big_endian_bits_helper(vec![false, true], "1");
    from_big_endian_bits_helper(vec![false, false, false, false, false, true, true, false],
                                "6");
    from_big_endian_bits_helper(vec![true, true, false, true, false, false, true], "105");
}

fn digits_u_helper<T: PrimUnsignedInt>(max_digit: &str) {
    let test = |radix, n, out| assert_eq!(format!("{:?}", digits_u(radix, n)), out);
    test(T::from_u8(2), T::from_u8(0), "[]");
    test(T::from_u8(3), T::from_u8(0), "[]");
    test(T::from_u8(8), T::from_u8(0), "[]");
    test(T::from_u8(10), T::from_u8(0), "[]");
    test(T::from_u8(12), T::from_u8(0), "[]");
    test(T::from_u8(57), T::from_u8(0), "[]");
    test(T::from_u8(2), T::from_u8(1), "[1]");
    test(T::from_u8(3), T::from_u8(1), "[1]");
    test(T::from_u8(8), T::from_u8(1), "[1]");
    test(T::from_u8(10), T::from_u8(1), "[1]");
    test(T::from_u8(12), T::from_u8(1), "[1]");
    test(T::from_u8(57), T::from_u8(1), "[1]");
    test(T::from_u8(2), T::from_u8(10), "[0, 1, 0, 1]");
    test(T::from_u8(3), T::from_u8(10), "[1, 0, 1]");
    test(T::from_u8(8), T::from_u8(10), "[2, 1]");
    test(T::from_u8(10), T::from_u8(10), "[0, 1]");
    test(T::from_u8(12), T::from_u8(10), "[10]");
    test(T::from_u8(57), T::from_u8(10), "[10]");
    test(T::from_u8(2), T::from_u8(107), "[1, 1, 0, 1, 0, 1, 1]");
    test(T::from_u8(3), T::from_u8(107), "[2, 2, 2, 0, 1]");
    test(T::from_u8(8), T::from_u8(107), "[3, 5, 1]");
    test(T::from_u8(10), T::from_u8(107), "[7, 0, 1]");
    test(T::from_u8(12), T::from_u8(107), "[11, 8]");
    test(T::from_u8(57), T::from_u8(107), "[50, 1]");
    test(T::max_value(), T::from_u8(0), "[]");
    test(T::max_value(), T::from_u8(107), "[107]");
    test(T::max_value(), T::max_value() - T::from_u8(1), max_digit);
    test(T::max_value(), T::max_value(), "[0, 1]");
}

#[test]
fn test_digits_u() {
    digits_u_helper::<u8>("[254]");
    digits_u_helper::<u16>("[65534]");
    digits_u_helper::<u32>("[4294967294]");
    digits_u_helper::<u64>("[18446744073709551614]");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn digits_u8_fail_1() {
    digits_u::<u8>(1, 10);
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn digits_u8_fail_2() {
    digits_u::<u8>(0, 10);
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn digits_u16_fail_1() {
    digits_u::<u16>(1, 10);
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn digits_u16_fail_2() {
    digits_u::<u16>(0, 10);
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn digits_u32_fail_1() {
    digits_u::<u32>(1, 10);
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn digits_u32_fail_2() {
    digits_u::<u32>(0, 10);
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn digits_u64_fail_1() {
    digits_u::<u64>(1, 10);
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn digits_u64_fail_2() {
    digits_u::<u64>(0, 10);
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn digits_usize_fail_1() {
    digits_u::<usize>(1, 10);
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn digits_usize_fail_2() {
    digits_u::<usize>(0, 10);
}

fn digits_integer_helper(radix: &str, n: &str, out: &str) {
    assert_eq!(format!("{:?}",
                       digits_integer(&Integer::from_str(radix).unwrap(),
                                      &Integer::from_str(n).unwrap())),
               out);
}

#[test]
fn test_digits_integer() {
    digits_integer_helper("2", "0", "[]");
    digits_integer_helper("3", "0", "[]");
    digits_integer_helper("8", "0", "[]");
    digits_integer_helper("10", "0", "[]");
    digits_integer_helper("12", "0", "[]");
    digits_integer_helper("57", "0", "[]");
    digits_integer_helper("2", "1", "[1]");
    digits_integer_helper("3", "1", "[1]");
    digits_integer_helper("8", "1", "[1]");
    digits_integer_helper("10", "1", "[1]");
    digits_integer_helper("12", "1", "[1]");
    digits_integer_helper("57", "1", "[1]");
    digits_integer_helper("2", "10", "[0, 1, 0, 1]");
    digits_integer_helper("3", "10", "[1, 0, 1]");
    digits_integer_helper("8", "10", "[2, 1]");
    digits_integer_helper("10", "10", "[0, 1]");
    digits_integer_helper("12", "10", "[10]");
    digits_integer_helper("57", "10", "[10]");
    digits_integer_helper("2", "187", "[1, 1, 0, 1, 1, 1, 0, 1]");
    digits_integer_helper("3", "187", "[1, 2, 2, 0, 2]");
    digits_integer_helper("8", "187", "[3, 7, 2]");
    digits_integer_helper("10", "187", "[7, 8, 1]");
    digits_integer_helper("12", "187", "[7, 3, 1]");
    digits_integer_helper("57", "187", "[16, 3]");
}

fn digits_integer_fail_helper(radix: &str, n: &str) {
    digits_integer(&Integer::from_str(radix).unwrap(),
                   &Integer::from_str(n).unwrap());
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn digits_integer_fail_1() {
    digits_integer_fail_helper("1", "10");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn digits_integer_fail_2() {
    digits_integer_fail_helper("0", "10");
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
fn digits_integer_fail_3() {
    digits_integer_fail_helper("2", "-1");
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
fn digits_integer_fail_4() {
    digits_integer_fail_helper("0", "-1");
}

macro_rules! test_digits_padded {
    (
        $t: ty,
        $f: ident,
        $test: ident,
        $helper: ident,
        $fail_1: ident,
        $fail_2: ident,
        $max: expr,
        $max_digit: expr
    ) => {
        fn $helper(size: usize, radix: $t, n: $t, out: &str) {
            assert_eq!(format!("{:?}", $f(size, radix, n)), out);
        }

        #[test]
        fn $test() {
            $helper(0, 2, 0, "[]");
            $helper(0, 3, 0, "[]");
            $helper(0, 57, 0, "[]");
            $helper(0, 2, 1, "[]");
            $helper(0, 3, 1, "[]");
            $helper(0, 57, 1, "[]");
            $helper(0, 2, 10, "[]");
            $helper(0, 3, 10, "[]");
            $helper(0, 57, 10, "[]");
            $helper(0, 2, 107, "[]");
            $helper(0, 3, 107, "[]");
            $helper(0, 57, 107, "[]");
            $helper(1, 2, 0, "[0]");
            $helper(1, 3, 0, "[0]");
            $helper(1, 57, 0, "[0]");
            $helper(1, 2, 1, "[1]");
            $helper(1, 3, 1, "[1]");
            $helper(1, 57, 1, "[1]");
            $helper(1, 2, 10, "[0]");
            $helper(1, 3, 10, "[1]");
            $helper(1, 57, 10, "[10]");
            $helper(1, 2, 107, "[1]");
            $helper(1, 3, 107, "[2]");
            $helper(1, 57, 107, "[50]");
            $helper(2, 2, 0, "[0, 0]");
            $helper(2, 3, 0, "[0, 0]");
            $helper(2, 57, 0, "[0, 0]");
            $helper(2, 2, 1, "[1, 0]");
            $helper(2, 3, 1, "[1, 0]");
            $helper(2, 57, 1, "[1, 0]");
            $helper(2, 2, 10, "[0, 1]");
            $helper(2, 3, 10, "[1, 0]");
            $helper(2, 57, 10, "[10, 0]");
            $helper(2, 2, 107, "[1, 1]");
            $helper(2, 3, 107, "[2, 2]");
            $helper(2, 57, 107, "[50, 1]");
            $helper(8, 2, 0, "[0, 0, 0, 0, 0, 0, 0, 0]");
            $helper(8, 3, 0, "[0, 0, 0, 0, 0, 0, 0, 0]");
            $helper(8, 57, 0, "[0, 0, 0, 0, 0, 0, 0, 0]");
            $helper(8, 2, 1, "[1, 0, 0, 0, 0, 0, 0, 0]");
            $helper(8, 3, 1, "[1, 0, 0, 0, 0, 0, 0, 0]");
            $helper(8, 57, 1, "[1, 0, 0, 0, 0, 0, 0, 0]");
            $helper(8, 2, 10, "[0, 1, 0, 1, 0, 0, 0, 0]");
            $helper(8, 3, 10, "[1, 0, 1, 0, 0, 0, 0, 0]");
            $helper(8, 57, 10, "[10, 0, 0, 0, 0, 0, 0, 0]");
            $helper(8, 2, 107, "[1, 1, 0, 1, 0, 1, 1, 0]");
            $helper(8, 3, 107, "[2, 2, 2, 0, 1, 0, 0, 0]");
            $helper(8, 57, 107, "[50, 1, 0, 0, 0, 0, 0, 0]");
            $helper(1, $max, 0, "[0]");
            $helper(1, $max, 107, "[107]");
            $helper(1, $max, $max - 1, $max_digit);
            $helper(2, $max, $max, "[0, 1]");
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
        fn $fail_1() {
            $f(3, 1 as $t, 10 as $t);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
        fn $fail_2() {
            $f(3, 0 as $t, 10 as $t);
        }
    }
}

test_digits_padded!(u8,
                    digits_padded_u,
                    test_digits_padded_u8,
                    digits_padded_u8_helper,
                    digits_padded_u8_fail_1,
                    digits_padded_u8_fail_2,
                    u8::max_value(),
                    "[254]");
test_digits_padded!(u16,
                    digits_padded_u,
                    test_digits_padded_u16,
                    digits_padded_u16_helper,
                    digits_padded_u16_fail_1,
                    digits_padded_u16_fail_2,
                    u16::max_value(),
                    "[65534]");
test_digits_padded!(u32,
                    digits_padded_u,
                    test_digits_padded_u32,
                    digits_padded_u32_helper,
                    digits_padded_u32_fail_1,
                    digits_padded_u32_fail_2,
                    u32::max_value(),
                    "[4294967294]");
test_digits_padded!(u64,
                    digits_padded_u,
                    test_digits_padded_u64,
                    digits_u64_padded_helper,
                    digits_u64_padded_fail_1,
                    digits_u64_padded_fail_2,
                    u64::max_value(),
                    "[18446744073709551614]");

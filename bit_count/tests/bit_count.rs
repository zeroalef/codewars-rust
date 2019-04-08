use bit_count;

#[test]
fn test_unsigned_ones() {
    for i in 1..63 {
        assert_eq!(bit_count::count_bits((1 as i64) << i), 1);
    }
}

#[test]
fn test_unsigned_twos() {
    for i in 1..62 {
        assert_eq!(bit_count::count_bits((3 as i64) << i), 2);
    }
}

#[test]
fn test_unsigned_threes() {
    for i in 1..61 {
        assert_eq!(bit_count::count_bits((7 as i64) << i), 3);
    }
}

#[test]
fn test_signed() {
    for i in 0..63 {
        assert_eq!(bit_count::count_bits((-2 as i64) << i), 63 - i);
    }
}

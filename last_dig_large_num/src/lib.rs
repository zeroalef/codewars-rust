#[cfg(test)]
mod tests {
    extern crate num;

    use num::bigint::BigInt;
    use num::ToPrimitive;
    use std::ops::Rem;
    use std::str::FromStr;

    fn last_digit(str1: &str, str2: &str) -> i32 {
        if str2.chars().all(|ch| (ch == '0')) {
            1
        } else if str1.chars().all(|ch| (ch == '0')) {
            0
        } else {
            match str1.chars().last() {
                Some('5') => 5,
                Some('6') => 6,
                Some('0') => 0,
                Some('1') => 1,
                Some(ch) => {
                    let f = |ch: char| -> i32 {
                        let n: BigInt = BigInt::from_str(str2).unwrap().rem(4);
                        let k: usize = n.to_usize().unwrap();
                        let ret_even: Vec<Vec<i32>> =
                            vec![vec![6, 2, 4, 8], vec![6, 4, 6, 4], vec![6, 8, 4, 2]];
                        let ret_odd: Vec<Vec<i32>> =
                            vec![vec![1, 3, 9, 7], vec![1, 7, 9, 3], vec![1, 9, 1, 9]];
                        match ch {
                            '2' => ret_even[0][k],
                            '4' => ret_even[1][k],
                            '8' => ret_even[2][k],
                            '3' => ret_odd[0][k],
                            '7' => ret_odd[1][k],
                            '9' => ret_odd[2][k],
                            _ => -1,
                        }
                    };
                    f(ch)
                }
                None => -1,
            }
        }
    }

    #[test]
    fn returns_expected() {
        assert_eq!(last_digit("4", "1"), 4);
        assert_eq!(last_digit("4", "2"), 6);
        assert_eq!(last_digit("9", "7"), 9);
        assert_eq!(last_digit("10", "10000000000"), 0);
        assert_eq!(
            last_digit(
                "1606938044258990275541962092341162602522202993782792835301376",
                "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"
            ),
            6
        );
        assert_eq!(
            last_digit(
                "3715290469715693021198967285016729344580685479654510946723",
                "68819615221552997273737174557165657483427362207517952651"
            ),
            7
        );
    }
}

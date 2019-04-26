#[cfg(test)]
mod tests {
    use std::fmt::Write;

    fn firstfac(x: i64) -> i64 {
        if x % 2 == 0 {
            return 2;
        };
        for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
            if x % n == 0 {
                return n;
            };
        }
        x
    }

    #[allow(clippy::match_bool)]
    fn prime_factors(n: i64) -> String {
        let mut ret: Vec<(i64, usize)> = Vec::new();
        let mut current = n;
        loop {
            let m = firstfac(current);
            match ret.iter().position(|&x| x.0 == m) {
                Some(i) => ret[i] = (m, ret[i].1 + 1),
                None => ret.push((m, 1)),
            }
            if m == current {
                break;
            } else {
                current /= m
            };
        }
        ret.iter().fold(String::new(), |mut acc, item| {
            match item.1 == 1 {
                true => write!(acc, "({})", item.0).ok(),
                false => write!(acc, "({}**{})", item.0, item.1).ok(),
            };
            acc
        })
    }

    fn testing(n: i64, exp: &str) {
        assert_eq!(&prime_factors(n), exp)
    }

    #[test]
    fn basics_prime_factors() {
        testing(7_775_460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    }
}

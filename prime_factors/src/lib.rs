#[cfg(test)]
mod tests {

    fn prime_factors(n: i64) -> String {
        let mut n = n as u64;
        let mut d = 2;
        let mut mem = std::collections::BTreeMap::new();
        while d <= n {
            if n % d == 0 {
                n /= d;
                let old = mem.entry(d).or_insert(0);
                *old += 1;
            } else {
                d += 1;
            }
      }
      mem.iter().map(|(key, val)| match *val {
          1 => format!("({})", key),
          _ => format!("({}**{})", key, val),
      }).collect::<String>()
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

#[cfg(test)]
mod tests {
    fn order_weight(s: &str) -> String {
        let mut s: Vec<&str> = s.split(' ').collect();
        s.sort_by(|a, b| {
            let left = a.chars().flat_map(|ch| ch.to_digit(10)).sum::<u32>();
            let right = b.chars().flat_map(|ch| ch.to_digit(10)).sum::<u32>();

            if left != right {
                left.cmp(&right)
            } else {
                a.cmp(&b)
            }
        });
        s.join(" ")
    }

    fn testing(s: &str, exp: &str)  {
        assert_eq!(order_weight(s), exp)
    }

    #[test]
    fn basics_order_weight() {
        testing("103 123 4444 99 2000", "2000 103 123 4444 99");
        testing(
            "2000 10003 1234000 44444444 9999 11 11 22 123",
            "11 11 2000 10003 22 123 1234000 44444444 9999",
        );
    }
}

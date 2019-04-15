#[cfg(test)]
mod tests {
    use super::*;

    fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
        if a1.is_empty() || a2.is_empty() {
            -1
        } else {
            let x_max = a1
                .iter()
                .map(|s| s.len() as i32)
                .fold(a1[0].len() as i32, |acc, i| std::cmp::max(acc, i));
            let x_min = a1
                .iter()
                .map(|s| s.len() as i32)
                .fold(a1[0].len() as i32, |acc, i| std::cmp::min(acc, i));
            let y_max = a2
                .iter()
                .map(|s| s.len() as i32)
                .fold(a2[0].len() as i32, |acc, i| std::cmp::max(acc, i));
            let y_min = a2
                .iter()
                .map(|s| s.len() as i32)
                .fold(a2[0].len() as i32, |acc, i| std::cmp::min(acc, i));
            println!("{} {} {} {}", x_max, x_min, y_min, y_max);
            std::cmp::max((x_max - y_min).abs(), (y_max - x_min).abs())
        }
    }

    fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
        println!("a1: {:?};", a1);
        println!("a2: {:?};", a2);
        let ans = mx_dif_lg(a1, a2);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut s1 = vec![
            "hoqq",
            "bbllkw",
            "oox",
            "ejjuyyy",
            "plmiis",
            "xxxzgpsssa",
            "xxwwkktt",
            "znnnnfqknaz",
            "qqquuhii",
            "dvvvwz",
        ];
        let mut s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, 13);
        s1 = vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ];
        s2 = vec!["bbbaaayddqbbrrrv"];
        dotest(s1, s2, 10);
    }
}

#[cfg(test)]
mod tests {
    fn perimeter(n: u64) -> u64 {
        (0..n)
            .fold((1u64, 1u64, 1u64), |a, _| (a.1, a.0 + a.1, a.1 + a.2))
            .2
            * 4
    }

    fn perimeter1(n: u64) -> u64 {
        let n = n as i32;
        let phi: f64 = 1.6180339887498948482;
        return 4 * (((phi.powi(n + 3) - (-phi).powi(-n - 3)) / 5_f64.sqrt()).round() as u64 - 1);
    }

    fn perimeter2(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 4,
            2 => 6,
            _ => {
                let mut v: Vec<u64> = vec![1, 1];
                for i in 2..=n as usize {
                    let tmp = v[i - 1] + v[i - 2];
                    v.push(tmp);
                }
                v.iter().sum::<u64>() * 4_u64
            }
        }
    }

    fn dotest(n: u64, exp: u64) -> () {
        assert_eq!(perimeter(n), exp);
        assert_eq!(perimeter1(n), exp);
        assert_eq!(perimeter2(n), exp);
    }

    #[test]
    fn basics_perimeter() {
        dotest(5, 80);
        dotest(7, 216);
        dotest(20, 114624);
        dotest(30, 14098308);
    }
}

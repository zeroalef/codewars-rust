#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
        let friends = frnds
            .iter()
            .filter_map(|&x| dist.get(fr_twns.get(x)?))
            .collect::<Vec<_>>();
        (friends[0]
            + friends
                .iter()
                .zip(friends.iter().skip(1))
                .map(|(&x, &y)| (y * y - x * x).sqrt())
                .sum::<f64>()
            + friends[friends.len() - 1]) as i32
    }

    fn testing(
        frnds: &[&str],
        fr_twns: HashMap<&str, &str>,
        dist: HashMap<&str, f64>,
        exp: i32,
    ) -> () {
        assert_eq!(tour(&frnds, fr_twns, dist), exp)
    }

    #[test]
    fn tests_tour() {
        let friends = ["A1", "A2", "A3", "A4", "A5"];
        let fr_towns: HashMap<&str, &str> =
            [("A1", "X1"), ("A2", "X2"), ("A3", "X3"), ("A4", "X4")]
                .iter()
                .cloned()
                .collect();
        let dst: HashMap<&str, f64> = [("X1", 100.0), ("X2", 200.0), ("X3", 250.0), ("X4", 300.0)]
            .iter()
            .cloned()
            .collect();
        testing(&friends, fr_towns, dst, 889);
    }
}

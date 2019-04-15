#[cfg(test)]
mod tests {
    fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
        let mut left = a.iter().map(|i| i * i).collect::<Vec<i64>>();
        let mut right = b.clone();

        left.sort();
        right.sort();

        (a.len() == b.len())
            && (left.iter().zip(right).all(|(x, y)| *x == y))
    }
    
    fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
        assert_eq!(comp(a, b), exp)
    }

    #[test]
    fn tests_comp() {

        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, true);
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, false);

    }
}

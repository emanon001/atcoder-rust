use cargo_snippet::snippet;

#[snippet("balanced_ternary")]
pub fn balanced_ternary(n: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut n = n;
    while n != 0 {
        let m = (n % 3 + 3) % 3;
        match m % 3 {
            0 => res.push(0),
            1 => {
                res.push(1);
                n -= 1;
            }
            2 => {
                res.push(-1);
                n += 1;
            }
            _ => unreachable!(),
        };
        n /= 3;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_balanced_ternary() {
        assert_eq!(balanced_ternary(0), vec![]);
        assert_eq!(balanced_ternary(1), vec![1]);
        assert_eq!(balanced_ternary(-1), vec![-1]);
        assert_eq!(balanced_ternary(2), vec![-1, 1]);
        assert_eq!(balanced_ternary(-2), vec![1, -1]);
        assert_eq!(balanced_ternary(3), vec![0, 1]);
        assert_eq!(balanced_ternary(-3), vec![0, -1]);
        assert_eq!(balanced_ternary(4), vec![1, 1]);
        assert_eq!(balanced_ternary(-4), vec![-1, -1]);
        assert_eq!(balanced_ternary(5), vec![-1, -1, 1]);
        assert_eq!(balanced_ternary(-5), vec![1, 1, -1]);
        assert_eq!(balanced_ternary(6), vec![0, -1, 1]);
        assert_eq!(balanced_ternary(-6), vec![0, 1, -1]);
        assert_eq!(balanced_ternary(7), vec![1, -1, 1]);
        assert_eq!(balanced_ternary(-7), vec![-1, 1, -1]);
        assert_eq!(balanced_ternary(8), vec![-1, 0, 1]);
        assert_eq!(balanced_ternary(-8), vec![1, 0, -1]);
        assert_eq!(balanced_ternary(9), vec![0, 0, 1]);
        assert_eq!(balanced_ternary(-9), vec![0, 0, -1]);
    }
}

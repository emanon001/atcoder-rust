use cargo_snippet::snippet;

#[snippet("bit")]
pub struct Bit {
    n: usize,
    data: Vec<i64>,
}

// [0, n)
#[snippet("bit")]
impl Bit {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0; n + 1],
        }
    }

    // 0-origin
    pub fn add(&mut self, i: usize, x: i64) {
        if i >= self.n {
            panic!();
        }
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] += x;
            i += ((i as isize) & -(i as isize)) as usize;
        }
    }

    // [0, i)
    pub fn sum(&self, i: usize) -> i64 {
        if i > self.n {
            panic!();
        }
        let mut i = i;
        let mut res = 0;
        while i > 0 {
            res += self.data[i];
            i -= ((i as isize) & -(i as isize)) as usize;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bit = Bit::new(3);
        assert_eq!(bit.sum(1), 0);
        assert_eq!(bit.sum(2), 0);
        assert_eq!(bit.sum(3), 0);
    }

    #[test]
    fn test_add_and_sum() {
        let mut bit = Bit::new(3);
        assert_eq!(bit.sum(1), 0);
        assert_eq!(bit.sum(2), 0);
        assert_eq!(bit.sum(3), 0);
        bit.add(0, 1);
        assert_eq!(bit.sum(1), 1);
        assert_eq!(bit.sum(2), 1);
        assert_eq!(bit.sum(3), 1);
        bit.add(1, 2);
        assert_eq!(bit.sum(1), 1);
        assert_eq!(bit.sum(2), 3);
        assert_eq!(bit.sum(3), 3);
        bit.add(2, 3);
        assert_eq!(bit.sum(1), 1);
        assert_eq!(bit.sum(2), 3);
        assert_eq!(bit.sum(3), 6);
        bit.add(0, -4);
        assert_eq!(bit.sum(1), -3);
        assert_eq!(bit.sum(2), -1);
        assert_eq!(bit.sum(3), 2);
    }

    #[test]
    fn test_sum_index_zero() {
        let mut bit = Bit::new(1);
        bit.add(0, 1);
        assert_eq!(bit.sum(0), 0);
        assert_eq!(bit.sum(1), 1);
    }
}

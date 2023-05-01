use cargo_snippet::snippet;

/// `T` is numeric only
#[snippet("bit")]
pub struct Bit<T>
where
    T: std::ops::AddAssign + std::ops::SubAssign + std::ops::Sub<Output = T> + num::Zero + Clone,
{
    n: usize,
    data: Vec<T>,
}

/// 0-origin
/// [0, n)
#[snippet("bit")]
impl<T> Bit<T>
where
    T: std::ops::AddAssign + std::ops::SubAssign + std::ops::Sub<Output = T> + num::Zero + Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![T::zero(); n + 1],
        }
    }

    /// 0-origin
    pub fn add(&mut self, i: usize, x: T) {
        if i >= self.n {
            panic!();
        }
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] += x.clone();
            i += ((i as isize) & -(i as isize)) as usize;
        }
    }

    /// 0-origin
    pub fn sub(&mut self, i: usize, x: T) {
        if i >= self.n {
            panic!();
        }
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] -= x.clone();
            i += ((i as isize) & -(i as isize)) as usize;
        }
    }

    /// [0, i)
    pub fn sum(&self, i: usize) -> T {
        if i > self.n {
            panic!();
        }
        let mut i = i;
        let mut res = T::zero();
        while i > 0 {
            res += self.data[i].clone();
            i -= ((i as isize) & -(i as isize)) as usize;
        }
        res
    }

    /// [i, j)
    pub fn range_sum(&self, i: usize, j: usize) -> T {
        if i > self.n || j > self.n {
            panic!();
        }
        if i >= j {
            return T::zero();
        }
        self.sum(j) - self.sum(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::{One, Zero};

    #[test]
    fn test_new() {
        let bit: Bit<i64> = Bit::new(3);
        assert_eq!(bit.sum(1), 0);
        assert_eq!(bit.sum(2), 0);
        assert_eq!(bit.sum(3), 0);
    }

    #[test]
    fn test_add_and_sum() {
        let mut bit: Bit<i64> = Bit::new(3);
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
        let mut bit: Bit<i64> = Bit::new(1);
        bit.add(0, 1);
        assert_eq!(bit.sum(0), 0);
        assert_eq!(bit.sum(1), 1);
    }

    #[test]
    fn test_sub_and_sum() {
        let mut bit: Bit<i64> = Bit::new(3);
        assert_eq!(bit.sum(1), 0);
        assert_eq!(bit.sum(2), 0);
        assert_eq!(bit.sum(3), 0);
        bit.sub(0, 1);
        assert_eq!(bit.sum(1), -1);
        assert_eq!(bit.sum(2), -1);
        assert_eq!(bit.sum(3), -1);
        bit.sub(1, 2);
        assert_eq!(bit.sum(1), -1);
        assert_eq!(bit.sum(2), -3);
        assert_eq!(bit.sum(3), -3);
        bit.sub(2, 3);
        assert_eq!(bit.sum(1), -1);
        assert_eq!(bit.sum(2), -3);
        assert_eq!(bit.sum(3), -6);
        bit.sub(0, -4);
        assert_eq!(bit.sum(1), 3);
        assert_eq!(bit.sum(2), 1);
        assert_eq!(bit.sum(3), -2);
    }

    #[test]
    fn test_range_sum() {
        let mut bit: Bit<i64> = Bit::new(3);
        bit.add(0, 1);
        bit.add(1, 2);
        bit.add(2, 3);
        bit.add(0, -4);
        assert_eq!(bit.range_sum(0, 0), 0); // i >= j
        assert_eq!(bit.range_sum(0, 1), -3);
        assert_eq!(bit.range_sum(0, 2), -1);
        assert_eq!(bit.range_sum(0, 3), 2);
        assert_eq!(bit.range_sum(1, 2), 2);
        assert_eq!(bit.range_sum(1, 3), 5);
        assert_eq!(bit.range_sum(2, 3), 3);
    }

    #[test]
    fn test_mod_int() {
        use crate::mod_int::ModInt;

        let mut bit: Bit<ModInt> = Bit::new(2);
        bit.add(0, ModInt::one());
        bit.add(1, ModInt::from(1_000_000_006));
        assert_eq!(bit.sum(1), ModInt::one());
        assert_eq!(bit.sum(2), ModInt::zero());
        assert_eq!(bit.range_sum(0, 1), ModInt::one());
        assert_eq!(bit.range_sum(0, 2), ModInt::zero());
        assert_eq!(bit.range_sum(1, 2), ModInt::from(1_000_000_006));
    }
}

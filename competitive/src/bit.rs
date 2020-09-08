use cargo_snippet::snippet;

#[snippet("bit")]
pub trait BitElement: Clone {
    fn bit_add_assign(&mut self, other: Self);
    fn bit_empty() -> Self;
}

#[snippet("bit")]
impl BitElement for i64 {
    fn bit_add_assign(&mut self, other: Self) {
        *self += other
    }

    fn bit_empty() -> Self {
        0
    }
}

#[snippet("bit")]
impl BitElement for i128 {
    fn bit_add_assign(&mut self, other: Self) {
        *self += other
    }

    fn bit_empty() -> Self {
        0
    }
}

#[snippet("bit")]
impl BitElement for f64 {
    fn bit_add_assign(&mut self, other: Self) {
        *self += other
    }

    fn bit_empty() -> Self {
        0.0
    }
}

#[snippet("bit")]
pub struct Bit<T: BitElement> {
    n: usize,
    data: Vec<T>,
}

/// [0, n)
#[snippet("bit")]
impl<T: BitElement> Bit<T> {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![T::bit_empty(); n + 1],
        }
    }

    /// 0-origin
    pub fn add(&mut self, i: usize, x: T) {
        if i >= self.n {
            panic!();
        }
        let mut i = i + 1;
        while i <= self.n {
            self.data[i].bit_add_assign(x.clone());
            i += ((i as isize) & -(i as isize)) as usize;
        }
    }

    /// [0, i)
    pub fn sum(&self, i: usize) -> T {
        if i > self.n {
            panic!();
        }
        let mut i = i;
        let mut res = T::bit_empty();
        while i > 0 {
            res.bit_add_assign(self.data[i].clone());
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
}

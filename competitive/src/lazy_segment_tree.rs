use crate::monoid::{Monoid};
use cargo_snippet::snippet;

#[snippet("lazy_segment_tree")]
#[snippet(include = "monoid")]
pub trait MapMonoid<F>: Monoid
{
    fn mmapping(&self, f: &F) -> Self;
    fn mfempty() -> F;
    fn mfappend(a: &F, b: &F) -> F;
}

#[snippet("lazy_segment_tree")]
pub struct LazySegmentTree<T, F>
where
    T: MapMonoid<F> + Clone,
    F: Clone
{
    size: usize,
    data: Vec<T>,
    lazy: Vec<F>,
}

#[snippet("lazy_segment_tree")]
impl<T, F> LazySegmentTree<T, F>
where
    T: MapMonoid<F> + Clone,
    F: Clone,
{
    pub fn new(size: usize) -> Self {
        let size = Self::normalize_data_size(size);
        let data = vec![T::mempty(); size * 2 - 1];
        let lazy = vec![T::mfempty(); size * 2 - 1];
        Self { size, data, lazy }
    }

    /// [a, b)
    /// 0-origin
    pub fn update(&mut self, a: usize, b: usize, v: F) {
        self.execute_update(a, b, &v, 0, 0, self.size);
    }

    /// [a, b)
    /// 0-origin
    pub fn query(&mut self, a: usize, b: usize) -> T {
        self.execute_query(a, b, 0, 0, self.size)
    }

    fn normalize_data_size(size: usize) -> usize {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        n
    }

    fn execute_update(&mut self, a: usize, b: usize, v: &F, i: usize, l: usize, r: usize) {
        self.eval(i);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            self.lazy[i] = T::mfappend(&self.lazy[i], &v);
            self.eval(i);
        } else {
            self.execute_update(a, b, v, i * 2 + 1, l, (l + r) / 2);
            self.execute_update(a, b, v, i * 2 + 2, (l + r) / 2, r);
            self.data[i] = self.data[i * 2 + 1].mappend(&self.data[i * 2 + 2]);
        }
    }

    fn execute_query(&mut self, a: usize, b: usize, i: usize, l: usize, r: usize) -> T {
        self.eval(i);
        if r <= a || b <= l {
            return T::mempty();
        }
        if a <= l && r <= b {
            self.data[i].clone()
        } else {
            let vl = self.execute_query(a, b, i * 2 + 1, l, (l + r) / 2);
            let vr = self.execute_query(a, b, i * 2 + 2, (l + r) / 2, r);
            vl.mappend(&vr)
        }
    }

    fn eval(&mut self, i: usize) {
        if i < self.size - 1 {
            self.lazy[i * 2 + 1] = T::mfappend(&self.lazy[i * 2 + 1], &self.lazy[i]);
            self.lazy[i * 2 + 2] = T::mfappend(&self.lazy[i * 2 + 2], &self.lazy[i]);
        }
        self.data[i] = self.data[i].mmapping(&self.lazy[i]);
        self.lazy[i] = T::mfempty();
    }
}

#[snippet("lazy_segment_tree")]
impl<T, F> From<&[T]> for LazySegmentTree<T, F>
where
    T: MapMonoid<F> + Clone,
    F: Clone,
{
    fn from(values: &[T]) -> Self {
        let mut st = LazySegmentTree::new(values.len());
        for (i, v) in values.into_iter().enumerate() {
            st.data[i + st.size - 1] = v.clone();
        }
        if st.size < 2 {
            return st;
        }
        for i in (0..=(st.size - 2)).rev() {
            st.data[i] = st.data[i * 2 + 1].mappend(&st.data[i * 2 + 2]);
        }
        st
    }
}

#[snippet("lazy_segment_tree")]
impl<T, F> From<Vec<T>> for LazySegmentTree<T, F>
where
    T: MapMonoid<F> + Clone,
    F: Clone,
{
    fn from(values: Vec<T>) -> Self {
        let values: &[T] = &values;
        Self::from(values)
    }
}

#[cfg(test)]
mod tests {
    use crate::monoid::{Monoid};
    use super::*;

    // sum
    impl Monoid for isize {
        fn mempty() -> Self {
            0
        }
        fn mappend(&self, other: &Self) -> Self {
            self + other
        }
    }

    // add 
    impl MapMonoid<isize> for isize {
        fn mmapping(&self, f: &isize) -> isize {
            self + f
        }
        fn mfempty() -> isize {
            0
        }
        fn mfappend(a: &isize, b: &isize) -> isize {
            a + b
        }
    }

    #[test]
    fn test_new() {
        let mut st: LazySegmentTree<isize, isize> = LazySegmentTree::new(3);
        assert_eq!(st.query(0, 3), 0);
    }

    #[test]
    fn test_from() {
        let v = vec![1, 3, 2];
        let mut st = LazySegmentTree::from(v);
        assert_eq!(st.query(0, 1), 1);
        assert_eq!(st.query(0, 2), 4);
        assert_eq!(st.query(0, 3), 6);
        assert_eq!(st.query(1, 2), 3);
        assert_eq!(st.query(1, 3), 5);
        assert_eq!(st.query(2, 3), 2);
    }

    #[test]
    fn test_from_size1() {
        let v = vec![1];
        let mut st = LazySegmentTree::from(v);
        assert_eq!(st.query(0, 1), 1);
    }

    #[test]
    fn test_update_and_query() {
        let mut st: LazySegmentTree<isize, isize> = LazySegmentTree::new(3);
        assert_eq!(st.query(0, 3), 0);
        st.update(0, 3, 1);
        st.update(1, 3, 2);
        st.update(2, 3, 3);
        // 1, 3, 6
        assert_eq!(st.query(0, 1), 1);
        assert_eq!(st.query(0, 2), 4);
        assert_eq!(st.query(0, 3), 10);
        assert_eq!(st.query(1, 2), 3);
        assert_eq!(st.query(1, 3), 9);
        assert_eq!(st.query(2, 3), 6);
    }
}

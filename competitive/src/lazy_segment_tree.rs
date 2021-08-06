use cargo_snippet::snippet;

#[snippet("lazy_segment_tree")]
pub trait Foo {
    fn mempty() -> Self;
    fn mappend(&self, other: &Self) -> Self;
}

#[snippet("lazy_segment_tree")]
#[snippet(include = "monoid")]
pub struct LazySegmentTree<T>
where
    T: Foo + Clone,
{
    size: usize,
    data: Vec<T>,
    lazy: Vec<Option<T>>,
}

#[snippet("lazy_segment_tree")]
impl<T> LazySegmentTree<T>
where
    T: Foo + Clone,
{
    pub fn new(size: usize) -> Self {
        let size = Self::normalize_data_size(size);
        let data = vec![T::mempty(); size * 2 - 1];
        let lazy = vec![None; size * 2 - 1];
        Self { size, data, lazy }
    }

    /// [a, b)
    /// 0-origin
    pub fn update(&mut self, a: usize, b: usize, v: T) {
        self.execute_update(a, b, v, 0, 0, self.size)
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

    fn execute_update(&mut self, a: usize, b: usize, v: T, i: usize, l: usize, r: usize) {
        self.eval(i);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            self.lazy[i] = Some(v.clone());
            self.eval(i);
        } else {
            self.execute_update(a, b, v.clone(), i * 2 + 1, l, (l + r) / 2);
            self.execute_update(a, b, v.clone(), i * 2 + 2, (l + r) / 2, r);
            self.data[i] = self.data[i * 2 + 1].clone().mappend(&self.data[i * 2 + 2]);
        }
    }

    fn execute_query(&mut self, a: usize, b: usize, i: usize, l: usize, r: usize) -> T {
        self.eval(i);
        if r <= a || b <= l {
            return T::mempty();
        }
        if a <= l && r <= b {
            return self.data[i].clone();
        }
        let vl = self.execute_query(a, b, i * 2 + 1, l, (l + r) / 2);
        let vr = self.execute_query(a, b, i * 2 + 2, (l + r) / 2, r);
        vl.mappend(&vr)
    }

    fn eval(&mut self, i: usize) {
        if self.lazy[i].is_none() {
            return;
        }
        let is_leaf = (i + 1) * 2 >= self.size;
        if !is_leaf {
            self.lazy[i * 2 + 1] = self.lazy[i].clone();
            self.lazy[i * 2 + 2] = self.lazy[i].clone();
        }
        self.data[i] = self.lazy[i].clone().unwrap();
        self.lazy[i] = None;
    }
}

#[snippet("lazy_segment_tree")]
impl<T> From<&[T]> for LazySegmentTree<T>
where
    T: Foo + Clone,
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
impl<T> From<Vec<T>> for LazySegmentTree<T>
where
    T: Foo + Clone,
{
    fn from(values: Vec<T>) -> Self {
        let values: &[T] = &values;
        Self::from(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // range add
    impl Foo for usize {
        fn mempty() -> Self {
            0
        }
        fn mappend(&self, other: &Self) -> Self {
            self + other
        }
    }

    #[test]
    fn test_new() {
        let mut st: LazySegmentTree<usize> = LazySegmentTree::new(3);
        assert_eq!(st.query(0, 3), 0);
    }

    #[test]
    fn test_from() {
        let v = vec![1, 3, 2];
        let mut st = LazySegmentTree::from(v);
        // assert_eq!(st.query(0, 1), 1);
        // assert_eq!(st.query(0, 2), 3);
        // assert_eq!(st.query(0, 3), 3);
        // assert_eq!(st.query(1, 2), 3);
        // assert_eq!(st.query(1, 3), 3);
        // assert_eq!(st.query(2, 3), 2);
    }

    #[test]
    fn tst_from_size1() {
        let v = vec![1];
        let mut st = LazySegmentTree::from(v);
        assert_eq!(st.query(0, 1), 1);
    }

    #[test]
    fn test_update_and_query() {
        let mut st: LazySegmentTree<usize> = LazySegmentTree::new(3);
        assert_eq!(st.query(0, 3), 0);
        // st.update(0, 3, 1);
        // st.update(1, 3, 2);
        // st.update(2, 3, 3);

        // assert_eq!(st.query(0, 1), 1);
        // assert_eq!(st.query(0, 2), 3);
        // assert_eq!(st.query(0, 3), 3);
        // assert_eq!(st.query(1, 2), 3);
        // assert_eq!(st.query(1, 3), 3);
        // assert_eq!(st.query(2, 3), 2);
    }
}

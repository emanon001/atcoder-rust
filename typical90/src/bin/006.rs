#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

pub trait Monoid {
    fn mempty() -> Self;
    fn mappend(&self, other: &Self) -> Self;
}
pub struct SegmentTree<T>
where
    T: Monoid + Clone,
{
    size: usize,
    data: Vec<T>,
}
impl<T> SegmentTree<T>
where
    T: Monoid + Clone,
{
    pub fn new(size: usize) -> Self {
        let size = Self::normalize_data_size(size);
        let data = vec![T::mempty(); size * 2 - 1];
        Self { size, data }
    }
    /// 0-origin
    pub fn update(&mut self, i: usize, v: T) {
        let mut i = i + self.size - 1;
        self.data[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = self.data[i * 2 + 1].mappend(&self.data[i * 2 + 2]);
        }
    }
    /// [a, b)
    /// 0-origin
    pub fn query(&self, a: usize, b: usize) -> T {
        self.execute_query(a, b, 0, 0, self.size)
    }
    fn normalize_data_size(size: usize) -> usize {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        n
    }
    fn execute_query(&self, a: usize, b: usize, i: usize, l: usize, r: usize) -> T {
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
}
impl<T> From<&[T]> for SegmentTree<T>
where
    T: Monoid + Clone,
{
    fn from(values: &[T]) -> Self {
        let mut st = SegmentTree::new(values.len());
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
impl<T> From<Vec<T>> for SegmentTree<T>
where
    T: Monoid + Clone,
{
    fn from(values: Vec<T>) -> Self {
        let values: &[T] = &values;
        Self::from(values)
    }
}

impl Monoid for (char, usize) {
    fn mempty() -> Self {
        (('z' as u8 + 1) as char, std::usize::MAX)
    }
    fn mappend(&self, other: &Self) -> Self {
        *self.min(other)
    }
}

fn solve() {
    input! {
        n: usize, k: usize,
        s: Chars
    };

    let st = SegmentTree::from((0..n).map(|i| (s[i], i)).collect::<Vec<_>>());
    let mut res = Vec::new();
    let mut l = 0;
    let mut r = n - k;
    for _ in 0..k {
        let (ch, i) = st.query(l, r + 1);
        res.push(ch);
        l = i + 1;
        r += 1;
    }
    println!("{}", res.iter().join(""));
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}

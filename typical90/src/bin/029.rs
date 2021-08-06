#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub trait LazySegmentTreeItem<U>: Clone
where
    U: Clone,
{
    fn empty_item() -> Self;
    fn append_item(&self, other: &Self) -> Self;
    fn update_item(&self, v: &U) -> Self;
    fn empty_update_value() -> U;
    fn append_update_value(a: &U, b: &U) -> U;
}
pub struct LazySegmentTree<T, U>
where
    T: LazySegmentTreeItem<U>,
    U: Clone,
{
    size: usize,
    data: Vec<T>,
    lazy: Vec<U>,
}
impl<T, U> LazySegmentTree<T, U>
where
    T: LazySegmentTreeItem<U> + Clone,
    U: Clone,
{
    pub fn new(size: usize) -> Self {
        let size = Self::normalize_data_size(size);
        let data = vec![T::empty_item(); size * 2 - 1];
        let lazy = vec![T::empty_update_value(); size * 2 - 1];
        Self { size, data, lazy }
    }
    /// [a, b)
    /// 0-origin
    pub fn update(&mut self, a: usize, b: usize, v: U) {
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
    fn execute_update(&mut self, a: usize, b: usize, v: &U, i: usize, l: usize, r: usize) {
        self.eval(i);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            self.lazy[i] = T::append_update_value(&self.lazy[i], &v);
            self.eval(i);
        } else {
            self.execute_update(a, b, v, i * 2 + 1, l, (l + r) / 2);
            self.execute_update(a, b, v, i * 2 + 2, (l + r) / 2, r);
            self.data[i] = self.data[i * 2 + 1].append_item(&self.data[i * 2 + 2]);
        }
    }
    fn execute_query(&mut self, a: usize, b: usize, i: usize, l: usize, r: usize) -> T {
        self.eval(i);
        if r <= a || b <= l {
            return T::empty_item();
        }
        if a <= l && r <= b {
            self.data[i].clone()
        } else {
            let vl = self.execute_query(a, b, i * 2 + 1, l, (l + r) / 2);
            let vr = self.execute_query(a, b, i * 2 + 2, (l + r) / 2, r);
            vl.append_item(&vr)
        }
    }
    fn eval(&mut self, i: usize) {
        if i < self.size - 1 {
            self.lazy[i * 2 + 1] = T::append_update_value(&self.lazy[i * 2 + 1], &self.lazy[i]);
            self.lazy[i * 2 + 2] = T::append_update_value(&self.lazy[i * 2 + 2], &self.lazy[i]);
        }
        self.data[i] = self.data[i].update_item(&self.lazy[i]);
        self.lazy[i] = T::empty_update_value();
    }
}
impl<T, F> From<&[T]> for LazySegmentTree<T, F>
where
    T: LazySegmentTreeItem<F> + Clone,
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
            st.data[i] = st.data[i * 2 + 1].append_item(&st.data[i * 2 + 2]);
        }
        st
    }
}
impl<T, F> From<Vec<T>> for LazySegmentTree<T, F>
where
    T: LazySegmentTreeItem<F> + Clone,
    F: Clone,
{
    fn from(values: Vec<T>) -> Self {
        let values: &[T] = &values;
        Self::from(values)
    }
}

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

impl LazySegmentTreeItem<usize> for usize {
    fn empty_item() -> Self {
        0
    }

    fn append_item(&self, other: &Self) -> Self {
        *self.max(other)
    }

    fn update_item(&self, v: &usize) -> Self {
        *self.max(v)
    }

    fn empty_update_value() -> usize {
        0
    }

    fn append_update_value(a: &usize, b: &usize) -> usize {
        *a.max(b)
    }
}

fn solve() {
    input! {
        w: usize, n: usize,
        lrv: [(usize, usize); n]
    };

    let mut st: LazySegmentTree<usize, usize> = LazySegmentTree::new(w + 1);
    for (l, r) in lrv {
        let h = st.query(l, r + 1) + 1;
        st.update(l, r + 1, h);
        println!("{}", h);
    }
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

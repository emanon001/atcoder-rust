#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub trait Monoid {
    fn empty() -> Self;
    fn append(&self, other: &Self) -> Self;
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
        let data = vec![T::empty(); size * 2 - 1];
        Self { size, data }
    }

    pub fn from_slice(values: &[T]) -> Self {
        let mut st = SegmentTree::new(values.len());
        for (i, v) in values.into_iter().enumerate() {
            st.data[i + st.size - 1] = v.clone();
        }
        if st.size < 2 {
            return st;
        }
        for i in (0..=(st.size - 2)).rev() {
            st.data[i] = st.data[i * 2 + 1].append(&st.data[i * 2 + 2]);
        }
        st
    }

    // 0-origin
    pub fn update(&mut self, i: usize, v: T) {
        let mut i = i + self.size - 1;
        self.data[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = self.data[i * 2 + 1].append(&self.data[i * 2 + 2]);
        }
    }

    // [a, b)
    // 0-origin
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
            return T::empty();
        }
        if a <= l && r <= b {
            return self.data[i].clone();
        }
        let vl = self.execute_query(a, b, i * 2 + 1, l, (l + r) / 2);
        let vr = self.execute_query(a, b, i * 2 + 2, (l + r) / 2, r);
        vl.append(&vr)
    }
}

impl Monoid for usize {
    fn empty() -> Self {
        0
    }

    fn append(&self, other: &Self) -> Self {
        *self | *other
    }
}

fn ch_to_usize(ch: char) -> usize {
    let pos = ((ch as u8) - 0x61) as u32;
    2.pow(pos) as usize
}

fn solve() {
    input! {
        _: usize,
        s: Chars,
        q: usize,
    };

    let s = s.into_iter().map(|ch| ch_to_usize(ch)).collect::<Vec<_>>();
    let mut st = SegmentTree::from_slice(&s);
    for _ in 0..q {
        input! { kind: usize };
        if kind == 1 {
            input! {
                i: Usize1, c: char
            };
            st.update(i, ch_to_usize(c));
        } else {
            input! {
                l: Usize1, r: Usize1
            };
            let mut x = st.query(l, r + 1);
            let mut res = 0_usize;
            while x > 0 {
                res += x % 2;
                x /= 2;
            }
            println!("{}", res);
        }
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

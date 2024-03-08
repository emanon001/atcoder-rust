#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub trait Monoid {
    fn mempty() -> Self;
    fn mappend(&self, other: &Self) -> Self;
}

impl Monoid for HashMap<usize, usize> {
    fn mempty() -> Self {
        HashMap::new()
    }
    fn mappend(&self, other: &Self) -> Self {
        let mut count_map = BTreeMap::new();
        for (k, v) in self {
            *count_map.entry(*k).or_insert(0) += *v;
        }
        for (k, v) in other {
            *count_map.entry(*k).or_insert(0) += *v;
        }
        count_map
            .into_iter()
            .rev()
            .take(2)
            .collect::<HashMap<_, _>>()
    }
}

#[derive(Clone, Debug)]
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
    /// 0-origin
    pub fn get(&self, a: usize) -> T {
        self.query(a, a + 1)
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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, Q: usize,
        A: [usize; N],
    };

    let mut v = vec![];
    for a in A {
        let mut map = HashMap::new();
        map.insert(a, 1_usize);
        v.push(map);
    }
    let mut st = SegmentTree::from(v);
    for _ in 0..Q {
        input_interactive! {
            kind: usize,
        };
        match kind {
            1 => {
                input_interactive! {
                    p: Usize1, x: usize,
                }
                let mut map = HashMap::new();
                map.insert(x, 1_usize);
                st.update(p, map);
            }
            2 => {
                input_interactive! {
                    l: Usize1, r: Usize1,
                }
                let map = st.query(l, r + 1);
                let ans = if map.len() < 2 {
                    0
                } else {
                    let key = map.keys().sorted().collect_vec()[0];
                    map[key]
                };
                println!("{}", ans);
            }
            _ => unreachable!(),
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

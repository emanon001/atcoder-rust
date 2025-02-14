#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

/// `T` is numeric only
pub struct Bit<T>
where
    T: std::ops::AddAssign + std::ops::SubAssign + std::ops::Sub<Output = T> + num::Zero + Clone,
{
    n: usize,
    data: Vec<T>,
}
/// 0-origin
/// [0, n)
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

pub trait BinarySearchOk<T>: PartialEq + Copy {
    fn bs_needs_next_search(&self, ng: &T) -> bool;
    fn bs_mid_value(&self, ng: &T) -> Self;
    fn bs_into(&self) -> T;
}
impl BinarySearchOk<i64> for i64 {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (self - ng).abs() > 1
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
impl BinarySearchOk<i128> for i128 {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (self - ng).abs() > 1
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
impl BinarySearchOk<f64> for f64 {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (self - ng).abs() > 1.0
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2.0
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
impl BinarySearchOk<isize> for isize {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (*self - *ng).abs() > 1
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
pub fn bsearch<T, Num: BinarySearchOk<T>, F>(ok: Num, ng: T, pred: F) -> Option<Num>
where
    F: Fn(T) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while ok.bs_needs_next_search(&ng) {
        let mid = ok.bs_mid_value(&ng);
        if pred(mid.bs_into()) {
            ok = mid;
        } else {
            ng = mid.bs_into();
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        P: [Usize1; N],
    };

    let mut empty_cell_bit = Bit::new(N);
    for i in 0..N {
        empty_cell_bit.add(i, 1);
    }

    let mut v = vec![0; N];
    for (a, p) in P.into_iter().enumerate().rev() {
        let pos = bsearch(N as isize, -1 as isize, |x| {
            let j = x as usize + 1;
            empty_cell_bit.sum(j) >= p + 1
        })
        .expect("exists pos");
        let pos = pos as usize;
        empty_cell_bit.sub(pos, 1);
        v[pos] = a + 1;
    }

    println!("{}", v.iter().join(" "));
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

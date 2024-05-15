#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

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
        mut A: [i64; N],
    };

    A.sort();

    let mut cusum = vec![0; N];
    cusum[0] = A[0];
    for i in 1..N {
        cusum[i] = cusum[i - 1] + A[i];
    }

    let M = 100000000;
    let mut ans = 0;
    for i in 0..N - 1 {
        let mid = bsearch(i as isize, N as isize, |x| {
            let j = x as usize;
            A[i] + A[j] < M
        }).unwrap_or(i as isize) as usize;

        let c = (mid - i) as i64;
        ans += cusum[mid] - cusum[i] + A[i] * c;

        let c = (N - mid - 1) as i64;
        ans += (cusum[N - 1] - cusum[mid] + A[i] * c) - M * c;
    }
    println!("{}", ans);
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

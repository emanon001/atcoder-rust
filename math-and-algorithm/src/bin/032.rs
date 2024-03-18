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
        N: usize, X: i64,
        mut A: [i64; N],
    };

    A.sort();
    let is_ok = A[bsearch(-1, N as isize, |i| {
        let i = i as usize;
        A[i] <= X
    })
    .unwrap_or(0) as usize]
        == X;
    let ans = if is_ok { "Yes" } else { "No" };
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

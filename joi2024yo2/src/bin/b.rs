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
        N: usize, M: usize, Q: usize,
        PA: [(u64, Usize1); N],
        TLR: [(Usize1, Usize1, Usize1); Q],
    };

    let mut cusum = vec![0_u64; N + 1];
    for i in 0..N {
        cusum[i + 1] = cusum[i] + PA[i].0;
    }
    let mut a_indexes = vec![vec![]; M];
    let mut a_cusum = vec![vec![0]; M];
    for i in 0..N {
        let (p, a) = PA[i];
        a_indexes[a].push(i);
        let sum = a_cusum[a][a_cusum[a].len() - 1];
        a_cusum[a].push(sum + p);
    }
    for (t, l, r) in TLR {
        let sum = cusum[r + 1] - cusum[l];
        let l = bsearch(a_indexes[t].len() as isize, -1, |x| {
            let x = x as usize;
            a_indexes[t][x] >= l
        });
        let r = bsearch(-1, a_indexes[t].len() as isize, |x| {
            let x = x as usize;
            a_indexes[t][x] <= r
        });
        let ans = sum - match (l, r) {
            (Some(l), Some(r)) => {
                (a_cusum[t][r as usize + 1] - a_cusum[t][l as usize]) / 2
            },
            _ => {
                0
            }
        };
        println!("{}", ans);
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

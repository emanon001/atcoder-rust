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
        N: usize, T: i64,
        S: Chars,
        mut X: [i64; N],
    };
    println!("{}", solve_inner(S, X, T));
}

#[allow(non_snake_case)]
fn solve_inner(S: Vec<char>, X: Vec<i64>, T: i64) -> usize {
    let mut l_pos = vec![];
    let mut r_pos = vec![];
    for (i, &ch) in S.iter().enumerate() {
        if ch == '0' {
            l_pos.push(X[i]);
        } else {
            r_pos.push(X[i]);
        }
    }
    l_pos.sort();
    r_pos.sort();
    let mut ans = 0_usize;
    for &pos in &l_pos {
        // 最寄りの右向きの位置を取得
        let j = bsearch(-1, r_pos.len() as isize, |j| r_pos[j as usize] < pos);

        ans += if let Some(j) = j {
            bsearch(j as isize + 1, -1, |k| {
                (r_pos[k as usize] - pos).abs() <= T * 2
            })
            .map(|k| j - k + 1)
            .unwrap_or(0)
        } else {
            0
        } as usize;
    }
    for &pos in &r_pos {
        // 最寄りの左向きの位置を取得
        let j = bsearch(l_pos.len() as isize, -1, |j| l_pos[j as usize] > pos);
        ans += if let Some(j) = j {
            bsearch(j as isize - 1, l_pos.len() as isize, |k| {
                (l_pos[k as usize] - pos).abs() <= T * 2
            })
            .map(|k| k - j + 1)
            .unwrap_or(0)
        } else {
            0
        } as usize;
    }
    ans / 2
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

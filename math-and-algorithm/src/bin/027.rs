#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(clippy::if_same_then_else)]
fn merge(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut res = vec![];
    let mut i = 0;
    let mut j = 0;
    while i < a.len() || j < b.len() {
        if i == a.len() {
            res.push(b[j]);
            j += 1;
        } else if j == b.len() {
            res.push(a[i]);
            i += 1;
        } else if a[i] < b[j] {
            res.push(a[i]);
            i += 1;
        } else {
            res.push(b[j]);
            j += 1;
        }
    }
    res
}

fn merge_sort(a: &[usize]) -> Vec<usize> {
    if a.len() <= 1 {
        return a.to_vec();
    }

    let mid = a.len() / 2;
    let l = merge_sort(&a[..mid]);
    let r = merge_sort(&a[mid..]);
    merge(&l, &r)
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [usize; N],
    };

    let ans = merge_sort(&A);
    println!("{}", ans.iter().join(" "));
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

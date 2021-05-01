#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        abcdev: [(i64, i64, i64, i64, i64); n]
    };

    let mut v1 = abcdev.clone().into_iter().enumerate().collect::<Vec<_>>();
    v1.sort_by_key(|&(i, (a, b, c, d, e))| a);
    let mut v2 = abcdev.clone().into_iter().enumerate().collect::<Vec<_>>();
    v2.sort_by_key(|&(i, (a, b, c, d, e))| b);
    let mut v3 = abcdev.clone().into_iter().enumerate().collect::<Vec<_>>();
    v3.sort_by_key(|&(i, (a, b, c, d, e))| c);
    let mut v4 = abcdev.clone().into_iter().enumerate().collect::<Vec<_>>();
    v4.sort_by_key(|&(i, (a, b, c, d, e))| d);
    let mut v5 = abcdev.clone().into_iter().enumerate().collect::<Vec<_>>();
    v5.sort_by_key(|&(i, (a, b, c, d, e))| e);

    let mut res = 0;
    for i in (0..n - 2).rev() {
        for j in (i + 1..n - 1).rev() {
            for k in (j + 1..n).rev() {
                let x1 = abcdev[i].0.max(abcdev[j].0).max(abcdev[k].0);
                let x2 = abcdev[i].1.max(abcdev[j].1).max(abcdev[k].1);
                let x3 = abcdev[i].2.max(abcdev[j].2).max(abcdev[k].2);
                let x4 = abcdev[i].3.max(abcdev[j].3).max(abcdev[k].3);
                let x5 = abcdev[i].4.max(abcdev[j].4).max(abcdev[k].4);
                let x = x1.min(x2).min(x3).min(x4).min(x5);
                if x < res {
                    break;
                } else {
                    res = x;
                }
            }
        }
    }
    println!("{}", res);
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

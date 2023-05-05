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
        n: usize
    };

    let mut a: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        let mut row = Vec::new();
        for j in 0..i + 1 {
            if j == 0 || j == i {
                row.push(1);
            } else {
                row.push(a[i - 1][j - 1] + a[i - 1][j]);
            }
        }
        a.push(row);
    }
    for row in a {
        println!("{}", row.iter().join(" "));
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

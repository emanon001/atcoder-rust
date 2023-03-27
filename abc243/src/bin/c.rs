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
        xyv: [(i64, i64); n],
        s: Chars
    };

    let mut y_map = HashMap::new();
    for i in 0..n {
        let (x, y) = xyv[i];
        let ch = s[i];
        y_map.entry(y).or_insert(Vec::new()).push((x, ch));
    }
    for (_, mut v) in y_map {
        if v.len() <= 1 {
            continue;
        }
        v.sort();
        let mut l = 10_i64.pow(10);
        for (x, c) in &v {
            if c == &'R' {
                l = *x;
                break;
            }
        }
        let mut r = -1;
        for (x, c) in v.iter().rev() {
            if c == &'L' {
                r = *x;
                break;
            }
        }
        if l <= r {
            println!("Yes");
            return;
        }
    }
    println!("No");
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

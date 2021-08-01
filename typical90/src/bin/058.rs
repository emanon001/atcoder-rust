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
        n: usize, k: usize,
    };

    let m = 10.pow(5);
    let p = 61;
    let mut next = vec![vec![0; m]; p];
    for x in 0..m {
        let v = (x + x.to_string().chars().map(|ch| ch.to_digit(10).unwrap() as usize).sum::<usize>()) % m;
        next[0][x] = v;
    }
    for i in 1..p {
        for x in 0..m {
            next[i][x] = next[i - 1][next[i - 1][x]];
        }
    }
    let mut rest = k;
    let mut cur = n;
    while rest > 0 {
        let mut i = 0;
        let mut c = 1;
        while c * 2 <= rest {
            c *= 2;
            i += 1;
        }
        rest -= c;
        cur = next[i][cur];
    }
    println!("{}", cur);
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

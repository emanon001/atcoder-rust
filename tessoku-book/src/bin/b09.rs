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
        rectangles: [(usize, usize, usize, usize); n]
    };

    let size = 1500;
    let mut cusum = vec![vec![0_i32; size + 2]; size + 2];

    for (a, b, c, d) in rectangles {
        let x1 = a + 1;
        let y1 = b + 1;
        let x2 = c;
        let y2 = d;
        cusum[y1][x1] += 1;
        cusum[y1][x2 + 1] -= 1;
        cusum[y2 + 1][x1] -= 1;
        cusum[y2 + 1][x2 + 1] += 1;
    }

    for i in 1..=size {
        for j in 1..=size {
            cusum[i][j] += cusum[i][j - 1];
        }
    }
    for j in 1..=size {
        for i in 1..=size {
            cusum[i][j] += cusum[i - 1][j];
        }
    }

    let mut res = 0_i32;
    for i in 1..=size {
        for j in 1..=size {
            let c = cusum[i][j];
            if c > 0 {
                res += 1;
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

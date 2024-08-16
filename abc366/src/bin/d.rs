#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [[i64; N]; N * N],
        Q: usize,
        L: [(usize, usize, usize, usize, usize, usize); Q]
    };

    let mut cusum = vec![vec![vec![0; N + 1]; N + 1]; N];
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                cusum[i][j + 1][k + 1] =
                    cusum[i][j][k + 1] + cusum[i][j + 1][k] - cusum[i][j][k] + A[i * N + j][k];
            }
        }
    }

    for (lx, rx, ly, ry, lz, rz) in L {
        let mut ans = 0;
        for i in lx..=rx {
            let i = i - 1;
            ans += cusum[i][ry][rz] + cusum[i][ly - 1][lz - 1]
                - cusum[i][ry][lz - 1]
                - cusum[i][ly - 1][rz];
        }
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

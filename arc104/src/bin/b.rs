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
        n: usize, s: Chars
    };

    let mut cusum = vec![vec![0; 4]; n + 1];
    for i in 0..n {
        let ch = s[i];
        let j = match ch {
            'A' => 0,
            'T' => 1,
            'C' => 2,
            'G' => 3,
            _ => unreachable!(),
        };
        for k in 0..4 {
            let c = if k == j { 1 } else { 0 };
            cusum[i + 1][k] = cusum[i][k] + c;
        }
    }
    let mut res = 0;
    for l in 0..n - 1 {
        for r in l + 2..n + 1 {
            let a_c = cusum[r][0] - cusum[l][0];
            let t_c = cusum[r][1] - cusum[l][1];
            let c_c = cusum[r][2] - cusum[l][2];
            let g_c = cusum[r][3] - cusum[l][3];
            if a_c == t_c && c_c == g_c {
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

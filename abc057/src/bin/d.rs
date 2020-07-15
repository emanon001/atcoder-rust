#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize, a: usize, b: usize,
        vv:[u64; n]
    };

    let mut dp1 = vec![vec![0_u64; n + 1]; n + 1];
    dp1[0][0] = 0;
    let mut dp2 = vec![vec![0_u64; n + 1]; n + 1];
    dp2[0][0] = 1;
    for i in 0..n {
        for j in 0..n {
            if dp1[i][j] > dp1[i + 1][j] {
                dp2[i + 1][j] = dp2[i][j];
            } else if dp1[i][j] == dp1[i + 1][j] {
                dp2[i + 1][j] += dp2[i][j];
            }
            dp1[i + 1][j] = dp1[i + 1][j].max(dp1[i][j]);
            dp1[i + 1][j + 1] = dp1[i][j] + vv[i];
            dp2[i + 1][j + 1] = dp2[i][j];
        }
    }
    let mut avg = 0_f64;
    for i in a..b + 1 {
        let v = (dp1[n][i]) as f64 / i as f64;
        if v > avg {
            avg = v;
        }
    }
    let mut count = 0_u64;
    for i in a..b + 1 {
        let v = (dp1[n][i]) as f64 / i as f64;
        if v == avg {
            count += dp2[n][i];
        }
    }
    println!("{}", avg);
    println!("{}", count);
}

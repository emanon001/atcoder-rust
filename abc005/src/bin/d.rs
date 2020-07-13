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
        n: usize,
        grid: [[isize; n]; n],
        q: usize,
        pv: [usize; q]
    };

    let mut cucusum = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            cucusum[i + 1][j + 1] =
                cucusum[i + 1][j] + cucusum[i][j + 1] - cucusum[i][j] + grid[i][j];
        }
    }

    for p in pv {
        let mut res = 0;
        for i in 0..n {
            for j in 0..n {
                for ib in i..n {
                    let t = i;
                    let b = ib;
                    let h = b - t + 1;
                    if h > p {
                        break;
                    }
                    let l = j;
                    let w = p / h;
                    let r = (j + w - 1).min(n - 1);
                    let sum = cucusum[b + 1][r + 1] - cucusum[b + 1][l] - cucusum[t][r + 1]
                        + cucusum[t][l];
                    if sum > res {
                        res = sum;
                    }
                }
            }
        }
        println!("{}", res);
    }
}

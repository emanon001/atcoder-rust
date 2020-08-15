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
        pv: [Usize1; n],
        cv: [i64; n],
    };

    let mut res: i64 = -(1_i64 << 60);
    for i in 0..n {
        let mut step = Vec::new();
        let mut loop_sum = 0;
        let mut j = pv[i];
        loop {
            loop_sum += cv[j];
            step.push(cv[j]);
            if i == j {
                break;
            }
            j = pv[j];
        }

        let no_loop_score = {
            let mut max = step[0];
            let mut cur = step[0];
            for k in 1..std::cmp::min(k, step.len()) {
                cur += step[k];
                if cur > max {
                    max = cur;
                }
            }
            max
        };

        let loop_count = if k >= step.len() {
            (k - step.len()) / step.len()
        } else {
            0
        };
        let loop_score = loop_sum * loop_count as i64;
        let rest_step = k - loop_count * step.len();
        let score = if loop_count == 0 {
            let mut max = step[0];
            let mut cur = step[0];
            for k in 1..rest_step {
                cur += step[k % step.len()];
                if cur > max {
                    max = cur;
                }
            }
            max
        } else {
            let mut max = 0;
            let mut cur = 0;
            for k in 0..rest_step {
                cur += step[k % step.len()];
                if cur > max {
                    max = cur;
                }
            }
            std::cmp::max(no_loop_score, loop_score + max)
        };
        if score > res {
            res = score;
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

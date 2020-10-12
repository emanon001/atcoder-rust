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
        n: usize, m: usize, q: usize,
    };

    let mut solves = vec![Vec::new(); n];
    let mut solve_counts = vec![0; m];
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    n_i: Usize1
                }
                let mut res = 0;
                for &i in &solves[n_i] {
                    res += n - solve_counts[i];
                }
                println!("{}", res);
            }
            2 => {
                input! {
                    n_i: Usize1,
                    m_i: Usize1,
                }
                solves[n_i].push(m_i);
                solve_counts[m_i] += 1;
            }
            _ => unreachable!(),
        };
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

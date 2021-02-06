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
        n: usize, m: usize,
        abv: [(Usize1, Usize1); m],
        k: usize,
        cdv: [(Usize1, Usize1); k],
    };

    let mut res = 0;
    for bits in 0..(1 << k) {
        let mut sara = vec![0; n];
        for i in 0..k {
            let (c, d) = cdv[i];
            if ((bits >> i) & 1) == 0 {
                sara[c] += 1;
            } else {
                sara[d] += 1;
            }
        }
        let mut count = 0;
        for i in 0..m {
            let (a, b) = abv[i];
            if sara[a] >= 1 && sara[b] >= 1 {
                count += 1;
            }
        }
        if count > res {
            res = count;
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

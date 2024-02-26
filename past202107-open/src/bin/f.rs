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
        DST: [(usize, usize, usize); N],
    };

    let mut imos = vec![0_isize; 10.pow(5) * 30];
    for (d, s, t) in DST {
        imos[d * 24 + s] += 1;
        imos[d * 24 + t] -= 1;
    }
    for i in 0..imos.len() - 1 {
        imos[i + 1] += imos[i];
    }
    let ans = if *imos.iter().max().unwrap() > 1 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
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

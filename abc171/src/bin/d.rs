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
        av: [i64; n],
        q: usize,
        bcv: [(i64, i64); q]
    };

    let mut table = HashMap::new();
    let mut sum = 0_i64;
    for a in av {
        sum += a;
        *table.entry(a).or_insert(0) += 1;
    }
    for (b, c) in bcv {
        let bc = *table.get(&b).unwrap_or(&0);
        table.insert(b, 0);
        let cc = *table.get(&c).unwrap_or(&0);
        table.insert(c, bc + cc);
        sum -= b * bc;
        sum += c * bc;
        println!("{}", sum);
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

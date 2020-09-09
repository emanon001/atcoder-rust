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
        n: usize, _: usize, p: usize, q: usize, r: usize,
        xyzv: [(Usize1, Usize1, i64); r]
    };

    let mut res = 0_i64;
    for girls in (0..n).combinations(p) {
        let girls = girls.into_iter().collect::<HashSet<_>>();
        let mut table = HashMap::new();
        for &(x, y, z) in &xyzv {
            if girls.contains(&x) {
                *table.entry(y).or_insert(0) += z;
            }
        }
        let mut values = table.values().collect::<Vec<_>>();
        values.sort_by_key(|&v| -v);
        let sum: i64 = values.into_iter().take(q).sum();
        if sum > res {
            res = sum;
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

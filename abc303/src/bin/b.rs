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
        av: [[Usize1; n]; m]
    };

    let mut set = HashSet::new();
    for i in 0..m {
        for j in 0..n - 1 {
            set.insert((av[i][j].min(av[i][j + 1]), av[i][j].max(av[i][j + 1])));
        }
    }

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            if !set.contains(&(i, j)) {
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

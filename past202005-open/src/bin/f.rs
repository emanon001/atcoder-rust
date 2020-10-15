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
        grid: [Chars; n]
    };

    let mut res = vec!['_'; n];
    for i in 0..n / 2 {
        let set1 = grid[i].iter().collect::<HashSet<_>>();
        let set2 = grid[n - i - 1].iter().collect::<HashSet<_>>();
        if let Some(&&ch) = set1.intersection(&set2).next() {
            res[i] = ch;
            res[n - i - 1] = ch;
        } else {
            println!("-1");
            return;
        }
    }
    if n % 2 == 1 {
        let i = n / 2;
        let ch = grid[i][0];
        res[i] = ch;
    }
    println!("{}", res.into_iter().collect::<String>());
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

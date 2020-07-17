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
        av: [usize; n]
    };

    let mut map = HashMap::new();
    for a in av {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut v = map.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|(_, c)| -(*c as isize));
    let res = n - v.into_iter().take(k).fold(0, |acc, (_, c)| acc + c);
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

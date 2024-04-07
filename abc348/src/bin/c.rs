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
        AC: [(i64, usize); N],
    };

    let mut min_map = HashMap::new();
    for (a, c) in AC {
        let e = min_map.entry(c).or_insert(a);
        *e = (*e).min(a);
    }
    let ans = min_map.into_iter().max_by_key(|(_, v)| *v).unwrap().1;
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

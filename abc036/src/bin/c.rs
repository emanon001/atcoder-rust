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
        av: [usize; n]
    };

    let mut map = BTreeMap::new();
    for i in 0..n {
        let v = av[i];
        map.entry(v).or_insert(Vec::new()).push(i);
    }
    let mut res = vec![0; n];
    for (a, (_, v)) in map.into_iter().enumerate() {
        for i in v {
            res[i] = a;
        }
    }
    for a in res {
        println!("{}", a);
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

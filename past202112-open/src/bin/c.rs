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
        pvv: [(char, String); n]
    };

    let mut res: HashMap<char, usize> = HashMap::new();
    for i in 0..n {
        let id = i + 1;
        let (p, v) = &pvv[i];
        if v == "AC" && !res.contains_key(p) {
            res.insert(*p, id);
        }
    }
    for p in vec!['A', 'B', 'C', 'D', 'E', 'F'] {
        println!("{}", res.get(&p).unwrap());
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

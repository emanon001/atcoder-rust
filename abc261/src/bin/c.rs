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
        sv: [String; n]
    };

    let mut map: HashMap<String, usize> = HashMap::new();
    for s in sv {
        if let Some(x) = map.get(&s) {
            println!("{}({})", s, x);
        } else {
            println!("{}", s);
        }
        *map.entry(s).or_insert(0) += 1;
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

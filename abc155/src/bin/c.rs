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

    let mut table: HashMap<String, usize> = HashMap::new();
    for s in sv {
        *table.entry(s).or_insert(0) += 1;
    }
    let max = *table.values().max().unwrap();
    let mut set = BTreeSet::new();
    for (s, c) in table {
        if c == max {
            set.insert(s);
        }
    }
    for s in set {
        println!("{}", s);
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

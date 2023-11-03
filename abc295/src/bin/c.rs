#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

pub fn count_list<T: Ord + std::hash::Hash + Clone>(
    list: Vec<T>,
) -> std::collections::HashMap<T, usize> {
    let mut map = std::collections::HashMap::new();
    for v in list {
        *map.entry(v).or_insert(0) += 1;
    }
    map
}

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        n: usize,
        a: [usize; n]
    };

    let ans: usize = count_list(a).into_values().map(|c| c / 2).sum();
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

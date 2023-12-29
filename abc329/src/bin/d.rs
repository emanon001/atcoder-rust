#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize, m: usize,
        a: [Usize1; m]
    };

    let mut counts = vec![0; n];
    let mut set = BTreeSet::new();
    for a_i in a {
        let cur_count = counts[a_i];
        let new_count = cur_count + 1;
        counts[a_i] = new_count;
        set.remove(&(cur_count, Reverse(a_i)));
        set.insert((cur_count + 1, Reverse(a_i)));
        let ans = set.iter().last().unwrap().1 .0 + 1;
        println!("{}", ans);
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

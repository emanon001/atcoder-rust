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
        AC: [(usize, usize); N],
    };

    let sorted = AC.iter().enumerate().sorted_by_key(|(_, &(a, _))| a).collect::<Vec<_>>();
    let mut c_set = BTreeSet::new();
    for (_, (_, c)) in sorted.iter() {
        c_set.insert(*c);
    }
    let mut ans = vec![];
    for (i, (_, c)) in sorted {
        c_set.remove(&c);
        match c_set.iter().next() {
            Some(c2) if c > c2 => {
                // ignore
            },
            _ => {
                ans.push(i + 1);
            }
        }
    }
    ans.sort();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
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

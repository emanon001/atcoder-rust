#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize,
    };

    let mut v = vec![];
    for i in 0..n {
        let i = i + 1;
        input_interactive! {
            c: usize,
            a: [usize; c]
        };
        v.push((i, c, a.into_iter().collect::<HashSet<_>>()));
    }
    input_interactive! {
        x: usize,
    };
    let filtered = v
        .into_iter()
        .filter(|(_, _, a)| a.contains(&x))
        .collect_vec();
    if filtered.is_empty() {
        println!("0");
        return;
    }
    let min_count = *filtered.iter().map(|(_, c, _)| c).min().unwrap();
    let filtered = filtered
        .into_iter()
        .filter(|(_, c, _)| c == &min_count)
        .collect_vec();
    println!("{}", filtered.len());
    println!("{}", filtered.iter().map(|(i, _, _)| i).join(" "));
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

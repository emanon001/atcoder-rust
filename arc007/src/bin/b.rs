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
        n: usize, m: usize,
        dv: [usize; m]
    };

    let mut p = 0;
    let mut disk_to_case = (1..=n)
        .into_iter()
        .map(|i| (i, i))
        .collect::<HashMap<_, _>>();
    for d in dv {
        if d == p {
            continue;
        }
        let case = disk_to_case[&d];
        disk_to_case.insert(p, case);
        disk_to_case.remove(&d);
        p = d;
    }
    let mut res = vec![0; n];
    for (d, c) in disk_to_case {
        res[c - 1] = d;
    }
    println!("{}", res.into_iter().join("\n"));
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

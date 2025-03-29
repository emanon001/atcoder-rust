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
        P: [usize; N],
    };

    let mut map = BTreeMap::new();
    for (i, p) in P.into_iter().enumerate() {
        map.entry(p).or_insert(Vec::new()).push(i);
    }

    let mut ans = vec![0; N];
    let mut rank = 1;
    for (_, users) in map.into_iter().rev() {
        let count = users.len();
        for user in users {
            ans[user] = rank;
        }
        rank += count;
    }
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

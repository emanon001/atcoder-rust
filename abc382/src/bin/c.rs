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
        N: usize, M: usize,
        A: [usize; N],
        B: [usize; M],
    };

    let mut v = vec![None; 2 * 10.pow(5) + 1];
    for (i, a) in A.into_iter().enumerate() {
        if v[a].is_none() {
            v[a] = Some(i as isize);
        }
    }
    let mut cur = None;
    for i in 0..v.len() {
        cur = match (cur, v[i]) {
            (None, _) => v[i],
            (Some(_), None) => cur,
            (Some(i), Some(j)) => Some(i.min(j)),
        };
        v[i] = cur;
    }
    eprintln!("{:?}", v.iter().take(10).collect::<Vec<_>>());

    let ans = B
        .into_iter()
        .map(|b| v[b].map(|it| it + 1).unwrap_or(-1))
        .join("\n");
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

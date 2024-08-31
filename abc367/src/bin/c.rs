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
        N: usize, K: usize,
        R: [usize; N],
    };

    let mut candidates: Vec<Vec<usize>> = vec![vec![]];
    for i in 0..N {
        let mut new_candidates = vec![];
        for x in 1..=R[i] {
            for c in &candidates {
                let mut c = c.clone();
                c.push(x);
                new_candidates.push(c);
            }
        }
        candidates = new_candidates;
    }
    let ans = candidates
        .into_iter()
        .filter(|c| {
            let sum = c.iter().sum::<usize>();
            sum % K == 0
        })
        .sorted()
        .collect::<Vec<_>>();
    for v in ans {
        println!("{}", v.iter().join(" "));
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

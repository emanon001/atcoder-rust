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
        _: usize, K: usize,
        A: [Usize1; K]
    };

    let diffs = A
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    let ans: usize = if K.is_even() {
        diffs.into_iter().step_by(2).sum()
    } else {
        let cusum1 = vec![0]
            .into_iter()
            .chain(diffs.iter().step_by(2).scan(0, |state, x| {
                *state += x;
                Some(*state)
            }))
            .collect_vec();
        let cusum2 = vec![0]
            .into_iter()
            .chain(diffs.iter().skip(1).step_by(2).scan(0, |state, x| {
                *state += x;
                Some(*state)
            }))
            .collect_vec();
        (0..cusum1.len())
            .map(|i| cusum1[i] + cusum2[cusum2.len() - 1] - cusum2[i])
            .min()
            .unwrap_or(0)
    };
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

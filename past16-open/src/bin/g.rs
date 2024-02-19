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
        A: [usize; 3 * N]
    };

    let ans = calc(&(0..3 * N).collect_vec(), &A) / factorial(N);
    println!("{}", ans);
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn calc(indexes: &[usize], av: &[usize]) -> usize {
    if indexes.is_empty() {
        return 1;
    }
    let mut res = 0;
    for i_comb in indexes.iter().combinations(3) {
        let idx = i_comb
            .clone()
            .into_iter()
            .sorted_by_key(|&i| av[*i])
            .collect_vec();
        if av[*idx[0]] + av[*idx[1]] <= av[*idx[2]] {
            continue;
        }
        let new_indexes = indexes
            .iter()
            .filter(|&x| x != idx[0] && x != idx[1] && x != idx[2])
            .copied()
            .collect_vec();
        res += calc(&new_indexes, av);
    }
    res
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

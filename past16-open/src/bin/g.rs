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

fn calc(indexes: &[usize], a: &[usize]) -> usize {
    if indexes.is_empty() {
        return 1;
    }
    let mut res = 0;
    for v in indexes.iter().combinations(3) {
        let v = v.clone().into_iter().sorted().collect_vec();
        if a[*v[0]] + a[*v[1]] <= a[*v[2]] {
            continue;
        }
        let new_indexes = indexes
            .iter()
            .filter(|&x| x != v[0] && x != v[1] && x != v[2])
            .copied()
            .collect_vec();
        res += calc(&new_indexes, a);
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

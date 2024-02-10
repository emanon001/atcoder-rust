#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        X: Chars,
        S: [Chars; N]
    };

    let x_comb = X.into_iter().combinations(3).collect_vec();
    let std::cmp::Reverse(ans) = S
        .into_iter()
        .enumerate()
        .map(|(i, s)| best_hand(&x_comb, &s, i + 1))
        .max()
        .unwrap()
        .2;
    println!("{}", ans);
}

fn best_hand(
    x_comb: &Vec<Vec<char>>,
    s: &[char],
    i: usize,
) -> (usize, std::cmp::Reverse<char>, std::cmp::Reverse<usize>) {
    let mut best = (0, std::cmp::Reverse('z'), std::cmp::Reverse(10.pow(5)));
    for x in x_comb {
        for s_comb in s.iter().combinations(2) {
            let hand = x
                .iter()
                .chain(s_comb)
                .counts()
                .into_iter()
                .map(|(ch, count)| (count, std::cmp::Reverse(*ch), std::cmp::Reverse(i)))
                .max()
                .unwrap();
            chmax!(best, hand);
        }
    }
    best
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

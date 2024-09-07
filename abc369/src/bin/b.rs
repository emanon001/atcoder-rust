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
        AS: [(isize, char); N]
    };

    let mut left = None;
    let mut right = None;
    let mut ans = 0;
    for (a, s) in AS {
        match (s, left, right) {
            ('L', None, _) => {
                left = Some(a);
            }
            ('L', Some(i), _) => {
                ans += (a - i).abs();
                left = Some(a);
            }
            ('R', _, None) => {
                right = Some(a);
            }
            ('R', _, Some(i)) => {
                ans += (a - i).abs();
                right = Some(a);
            }
            _ => unreachable!(),
        }
    }
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

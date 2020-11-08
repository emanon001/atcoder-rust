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
        n: Chars
    };

    let n = n
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();

    let mut res = std::isize::MAX;
    for bits in 0..(1 << n.len()) {
        if bits == 0 {
            continue;
        }
        let mut x = 0;
        let mut c = 0;
        for d in (0..n.len()).rev() {
            if (bits >> d) & 1 == 1 {
                x = x * 10 + n[n.len() - d - 1];
            } else {
                c += 1;
            }
        }
        if x % 3 == 0 && c < res {
            res = c;
        }
    }
    let res = if res == std::isize::MAX { -1 } else { res };
    println!("{}", res);
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

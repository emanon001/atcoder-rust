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
        av: [[usize]; m]
    };

    let mut ans = 0;
    for bits in 1..1 << m {
        let mut set = HashSet::new();
        for i in 0..m {
            if (bits >> i) & 1 == 1 {
                for a in &av[i] {
                    set.insert(a);
                }
            }
        }
        if set.len() == n {
            ans += 1;
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

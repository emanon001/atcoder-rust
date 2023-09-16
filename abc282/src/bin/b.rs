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
        grid: [Chars; n]
    };

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            let is_ok = (0..m).all(|m_i| grid[i][m_i] == 'o' || grid[j][m_i] == 'o');
            if is_ok {
                res += 1;
            }
        }
    }
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

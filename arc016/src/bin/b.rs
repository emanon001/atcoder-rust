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
        n: usize,
        music: [Chars; n]
    };

    let mut res = 0;
    for i in 0..9 {
        let mut prev = '_';
        for j in 0..n {
            let ch = music[j][i];
            res += match ch {
                'x' => 1,
                'o' => {
                    if prev != 'o' {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            };
            prev = ch;
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

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
        _n: usize,
        s: Chars
    };

    let mut res = Vec::new();
    let mut quote_count = 0;
    for ch in s {
        match ch {
            '"' => {
                quote_count += 1;
                res.push('"');
            }
            ',' => {
                res.push(if quote_count % 2 == 0 { '.' } else { ',' });
            }
            ch => {
                res.push(ch);
            }
        }
    }
    println!("{}", res.iter().join(""));
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

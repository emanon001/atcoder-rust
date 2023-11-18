#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut ans = Vec::new();
    for i in 0..n {
        ans.push(a[i]);
        if i == n - 1 {
            break;
        }
        let x = a[i];
        let y = a[i + 1];
        if x < y {
            for v in x + 1..y {
                ans.push(v);
            }
        } else if x > y {
            for v in (y + 1..x).rev() {
                ans.push(v);
            }
        }
    }
    println!("{}", ans.iter().join(" "));
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

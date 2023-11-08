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
        mut a: i64, mut b: i64
    };

    let mut ans = 0;
    while a != b {
        let max = a.max(b);
        let min = a.min(b);
        let diff = (max - min).abs();
        let mut c = diff / min;
        if diff % min != 0 {
            c += 1;
        }
        ans += c;
        if max == a {
            a -= b * c;
        } else {
            b -= a * c;
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

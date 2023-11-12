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
    };

    let mut ans = vec![None; n + 1];
    ans[1] = 0.into();
    ans[n] = 1.into();
    let mut l = 1_usize;
    let mut r = n;
    loop {
        if let (Some(v1), Some(v2)) = (ans[l], ans[l + 1]) {
            if v1 != v2 {
                println!("! {}", l);
                return;
            }
        }
        let m = (l + r) / 2;
        println!("? {}", m);
        input! {
            v: usize
        };
        ans[m] = v.into();
        eprintln!(
            "l: {}({:?}), m: {}({:?}), r: {}({:?})",
            l, ans[l], m, v, r, ans[r]
        );
        match (ans[l], v, ans[r]) {
            (Some(0), 0, Some(1)) => {
                l = m;
            }
            (Some(0), 1, Some(1)) => {
                r = m;
            }
            (Some(1), 0, Some(0)) => {
                r = m;
            }
            (Some(1), 1, Some(0)) => {
                l = m;
            }
            _ => unreachable!(),
        }
    }
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

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
        q: usize,
    };

    let mut que = VecDeque::new();
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    c: char, x: i64
                };
                que.push_back((c, x));
            }
            2 => {
                input! {
                    d: i64
                };
                let mut rest = d;
                let mut deleted = HashMap::new();
                while rest > 0 && !que.is_empty() {
                    let (c, x) = que.pop_front().unwrap();
                    let d_c = x.min(rest);
                    *deleted.entry(c).or_insert(0_i64) += d_c;
                    if x > rest {
                        que.push_front((c, x - rest));
                    }
                    rest -= d_c;
                }
                let res = deleted.values().map(|&v| v * v).sum::<i64>();
                println!("{}", res);
            }
            _ => unreachable!(),
        };
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

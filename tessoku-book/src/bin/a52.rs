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
        input! { kind: usize };
        match kind {
            1 => {
                input! { x: String };
                que.push_back(x);
            }
            2 => {
                println!("{}", que.iter().next().unwrap());
            }
            3 => {
                que.pop_front();
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

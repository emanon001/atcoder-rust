#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn main() {
    input_interactive! {
        Q: usize,
    };

    let mut deque = VecDeque::new();
    for _ in 0..Q {
        input_interactive! {
            kind: usize,
        };

        match kind {
            1 => {
                input_interactive! {
                    c: u64,
                    x: u64,
                };
                deque.push_back((c, x));
            }
            2 => {
                input_interactive! {
                    k: u64,
                };
                let mut ans = 0_u64;
                let mut count = 0_u64;
                while count < k {
                    let (c, x) = deque.pop_front().unwrap();
                    let add_count = c.min(k - count);
                    if add_count < c {
                        deque.push_front((c - add_count, x));
                    }
                    count += add_count;
                    ans += x * add_count;
                }
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}

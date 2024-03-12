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
fn solve() {
    input_interactive! {
        N: usize,
        A: [i64; N],
        Q: usize,
    };

    let mut head = A[0];
    let mut next_map = HashMap::new();
    let mut prev_map = HashMap::new();
    for i in 0..N {
        if i + 1 < N {
            next_map.insert(A[i], A[i + 1]);
        }
        if i > 0 {
            prev_map.insert(A[i], A[i - 1]);
        }
    }
    for _ in 0..Q {
        input_interactive! {
            kind: usize,
        };
        match kind {
            1 => {
                input_interactive! {
                    x: i64, y: i64,
                };
                let cur_next = next_map.get(&x).copied();
                next_map.insert(x, y);
                prev_map.insert(y, x);
                if let Some(cur_next) = cur_next {
                    next_map.insert(y, cur_next);
                    prev_map.insert(cur_next, y);
                }
            }
            2 => {
                input_interactive! {
                    x: i64,
                };
                let cur_prev = prev_map.get(&x).copied();
                let cur_next = next_map.get(&x).copied();
                next_map.remove(&x);
                prev_map.remove(&x);
                match (cur_prev, cur_next) {
                    (Some(cur_prev), Some(cur_next)) => {
                        next_map.insert(cur_prev, cur_next);
                        prev_map.insert(cur_next, cur_prev);
                    }
                    (Some(cur_prev), None) => {
                        next_map.remove(&cur_prev);
                    }
                    (None, Some(cur_next)) => {
                        prev_map.remove(&cur_next);
                        head = cur_next;
                    }
                    (None, None) => {
                        unreachable!()
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    let mut head = head;
    let mut ans = vec![head];
    while let Some(v) = next_map.get(&head).copied() {
        ans.push(v);
        head = v;
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

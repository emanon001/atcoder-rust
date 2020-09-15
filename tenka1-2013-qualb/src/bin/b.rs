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
        q: usize, l: i64,
    };

    let mut stack = Vec::new();
    let mut size = 0_i64;
    for _ in 0..q {
        input! { kind: String }
        match kind.as_ref() {
            "Push" => {
                input! { n: i64, m: i64 }
                if size + n > l {
                    println!("FULL");
                    return;
                }
                size += n;
                stack.push((m, n));
            }
            "Pop" => {
                input! { n: i64 }
                if n > size {
                    println!("EMPTY");
                    return;
                }
                let mut rest = n;
                while rest > 0 {
                    let (x, y) = stack.pop().unwrap();
                    if y > rest {
                        stack.push((x, y - rest));
                    }
                    rest -= rest.min(y);
                }
                size -= n;
            }
            "Top" => {
                if let Some((m, _)) = stack.last() {
                    println!("{}", m);
                } else {
                    println!("EMPTY");
                    return;
                }
            }
            "Size" => {
                println!("{}", size);
            }
            _ => unreachable!(),
        }
    }
    println!("SAFE");
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

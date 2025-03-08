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
        Q: usize,
    };

    let mut stack = vec![0].repeat(100);
    for _ in 0..Q {
        input_interactive! {
            t: usize,
        };

        match t {
            1 => {
                input_interactive! {
                    x: usize,
                };
                stack.push(x);
            }
            2 => {
                let x = stack.pop().unwrap();
                println!("{}", x);
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

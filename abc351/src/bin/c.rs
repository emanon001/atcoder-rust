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
        A: [usize; N],
    };

    let mut v = vec![];
    for a in A {
        v.push(a);
        loop {
            if v.len() <= 1 {
                break;
            }
            if v[v.len() - 1] != v[v.len() - 2] {
                break;
            }
            let b = v[v.len() - 1];
            v.pop();
            v.pop();
            v.push(b + 1);
        }
    }
    eprintln!("{:?}", v);
    println!("{}", v.len());
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

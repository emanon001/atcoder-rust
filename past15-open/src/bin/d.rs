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
        A: i64, B: i64, C: i64, D: i64, R: i64
    };

    let check = || {
        let mut times = HashSet::new();
        let mut t = 0;
        while t <= 3000 {
            let (p, q) = if t < B { (A, A + R) } else { (C, C + R) };
            if t < q {
                for t2 in p.max(t)..q {
                    times.insert(t2);
                }
            }
            t += D;
        }
        (C..C + R).all(|t| times.contains(&t))
    };

    let ans = if check() { "Yes" } else { "No" };
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

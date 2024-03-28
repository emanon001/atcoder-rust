#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input! {
        N: usize, L: isize, T: isize, X: isize,
        AB: [(isize, isize); N],
    };

    let mut ans = 0;
    let mut high_load_times = 0;
    let mut i = 0;
    while i < N {
        let (a, b) = AB[i];
        if b >= L && a > T {
            println!("forever");
            return;
        }
        if b >= L {
            match (high_load_times + a).cmp(&T) {
                std::cmp::Ordering::Less => {
                    ans += a;
                    high_load_times += a;
                }
                std::cmp::Ordering::Equal => {
                    ans += a;
                    ans += X;
                    high_load_times = 0;
                }
                std::cmp::Ordering::Greater => {
                    ans += T - high_load_times;
                    ans += X;
                    high_load_times = 0;
                    continue;
                }
            }
        } else {
            ans += a;
            high_load_times = 0;
        }
        i += 1;
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

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
        n: usize, k: i64,
        av: [i64; n]
    };

    let m = n / 2;
    let mut set1 = HashSet::new();
    for bits in 0..1 << m {
        let mut sum = 0_i64;
        for i in 0..m {
            if (bits >> i) & 1 == 1 {
                sum += av[i];
            }
        }
        set1.insert(sum);
    }
    let mut set2 = HashSet::new();
    for bits in 0..1 << (n - m) {
        let mut sum = 0_i64;
        for i in 0..(n - m) {
            if (bits >> i) & 1 == 1 {
                sum += av[m + i];
            }
        }
        set2.insert(sum);
    }
    for a in set1 {
        let b = k - a;
        if set2.contains(&b) {
            println!("Yes");
            return;
        }
    }
    println!("No");
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

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn divisors(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut x = 1;
    while x * x <= n {
        if n % x == 0 {
            res.push(x);
            let y = n / x;
            if y != x {
                res.push(y);
            }
        }
        x += 1;
    }
    res
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [u64; N],
    };

    let mut count_map = HashMap::new();
    for &a in &A {
        *count_map.entry(a).or_insert(0_u64) += 1;
    }
    let mut ans = 0_u64;
    for x in 1_u64..=10.pow(5) * 2 {
        let ds = divisors(x);
        let mut ds_set = HashSet::new();
        for i in 0..ds.len() {
            for j in i..ds.len() {
                ds_set.insert(ds[i] * ds[j]);
            }
        }
        let x = x * x;
        // eprintln!("{}, {:?}", x, ds_set);
        for d in ds_set {
            match (count_map.get(&d), count_map.get(&(x / &d))) {
                (Some(&c1), Some(&c2)) => {
                    let add = if d == x / d { c1 * (c1 - 1) } else { c1 * c2 };
                    // eprintln!("x: {}, d: {}, add: {}", x, d, add);
                    ans += add;
                }
                _ => {}
            }
        }
    }
    ans /= 2;
    if let Some(c) = count_map.get(&0) {
        ans += c * (c - 1) / 2;
        ans += c * (N as u64 - c);
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

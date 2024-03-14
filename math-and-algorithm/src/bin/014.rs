#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn prime_factor(n: u64) -> std::collections::HashMap<u64, u64> {
    if n < 2 {
        return std::collections::HashMap::new();
    }
    let mut res = std::collections::HashMap::new();
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            let c = res.entry(i).or_insert(0);
            *c += 1;
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        res.insert(n, 1);
    }
    res
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: u64,
    };

    let ans = prime_factor(N)
        .into_iter()
        .flat_map(|(k, v)| vec![k; v as usize])
        .sorted()
        .join(" ");
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

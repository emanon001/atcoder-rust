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
        N: usize, M: usize, K: usize,
        C: [([Usize1], char); M],
    };

    let mut ans = 0;
    for bits in 0..1 << N {
        let mut ok = true;
        for (v, r) in &C {
            let mut count = 0;
            for &i in v {
                if bits >> i & 1 == 1 {
                    count += 1;
                }
            }
            ok = ok
                && match (count >= K, r) {
                    (true, 'o') => true,
                    (false, 'x') => true,
                    _ => false,
                };
        }
        if ok {
            ans += 1;
        }
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

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
        B: [usize; N],
        C: [usize; N],
    };

    let _mod = 46;

    let mut m64_a = vec![0_u64; _mod];
    for a in A {
        m64_a[a % _mod] += 1;
    }
    let mut m64_b = vec![0_u64; _mod];
    for b in B {
        m64_b[b % _mod] += 1;
    }
    let mut m64_c = vec![0_u64; _mod];
    for c in C {
        m64_c[c % _mod] += 1;
    }

    let mut ans = 0;
    for a in 0.._mod {
        for b in 0.._mod {
            let ab = (a + b) % _mod;
            let c = (_mod - ab) % _mod;
            ans += m64_a[a] * m64_b[b] * m64_c[c];
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

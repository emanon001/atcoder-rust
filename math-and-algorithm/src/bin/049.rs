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
    };

    let m = 1000000007;
    let mut fib = vec![0; N];
    fib[0] = 1;
    fib[1] = 1;
    for i in 2..N {
        fib[i] = (fib[i - 1] + fib[i - 2]) % m;
    }
    println!("{}", fib[N - 1]);
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

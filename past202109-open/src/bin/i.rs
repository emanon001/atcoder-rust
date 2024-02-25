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
        A: [u64; N],
    };

    let mut two_count = 0;
    let mut av = BTreeSet::new();
    for (i, a) in A.into_iter().enumerate() {
        let mut a = a;
        while a % 2 == 0 {
            two_count += 1;
            a /= 2;
        }
        av.insert((a, i));
    }
    for _ in 0..two_count {
        let &item = av.iter().next().unwrap();
        av.remove(&item);
        let (a, i) = item;
        av.insert((a * 3, i));
    }
    let ans = av.iter().next().unwrap().0;
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

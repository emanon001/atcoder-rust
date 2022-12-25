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
        n: usize, x: Usize1,
        mut av: Chars
    };

    let mut que = VecDeque::new();
    que.push_back(x);
    while let Some(pos) = que.pop_front() {
        av[pos] = '@';
        if pos > 0 && av[pos - 1] == '.' {
            av[pos - 1] = '@';
            que.push_back(pos - 1);
        }
        if pos < n - 1 && av[pos + 1] == '.' {
            av[pos + 1] = '@';
            que.push_back(pos + 1);
        }
    }
    println!("{}", av.iter().join(""));
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

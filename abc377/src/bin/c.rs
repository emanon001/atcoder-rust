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
        N: u64, M: usize,
        AB: [(Usize1, Usize1); M],
    };

    let mut set = HashSet::new();
    for (i, j) in &AB {
        set.insert((*i, *j));
    }

    for (i, j) in AB {
        for (di, dj) in [
            (2, 1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
        ] {
            let new_i = i as isize + di;
            let new_j = j as isize + dj;
            if new_i < 0 || new_i >= N as isize || new_j < 0 || new_j >= N as isize {
                continue;
            }
            set.insert((new_i as usize, new_j as usize));
        }
    }

    let ans = N * N - set.len() as u64;
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

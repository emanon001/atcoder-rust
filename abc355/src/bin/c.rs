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
        N: usize, T: usize,
        A: [Usize1; T],
    };

    let mut horizontal = vec![HashSet::new(); N];
    let mut vertical = vec![HashSet::new(); N];
    for a in 0..N * N {
        let i = a / N;
        let j = a % N;
        horizontal[i].insert(j);
        vertical[j].insert(i);
    }
    let mut cross1 = HashSet::new();
    for i in 0..N {
        let j = i;
        cross1.insert((i, j));
    }
    let mut cross2 = HashSet::new();
    for i in 0..N {
        let j = N - 1 - i;
        cross2.insert((i, j));
    }

    for (turn, a) in A.into_iter().enumerate() {
        let turn = turn + 1;
        let i = a / N;
        let j = a % N;

        vertical[j].remove(&i);
        if vertical[j].is_empty() {
            println!("{}", turn);
            return;
        }

        horizontal[i].remove(&j);
        if horizontal[i].is_empty() {
            println!("{}", turn);
            return;
        }

        cross1.remove(&(i, j));
        if cross1.is_empty() {
            println!("{}", turn);
            return;
        }

        cross2.remove(&(i, j));
        if cross2.is_empty() {
            println!("{}", turn);
            return;
        }
    }
    println!("-1");
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

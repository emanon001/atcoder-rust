#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

type PR = (isize, isize, isize);

fn can_move(from: PR, to: PR) -> bool {
    !in_circle(from, to) && !out_circle(from, to)
}

fn in_circle(p1: PR, p2: PR) -> bool {
    let dx = (p1.0 - p2.0).abs();
    let dy = (p1.1 - p2.1).abs();
    let dr = (p1.2 - p2.2).abs();
    dx * dx + dy * dy < dr * dr
}

fn out_circle(p1: PR, p2: PR) -> bool {
    let dx = (p1.0 - p2.0).abs();
    let dy = (p1.1 - p2.1).abs();
    let dr = (p1.2 + p2.2).abs();
    dx * dx + dy * dy > dr * dr
}

fn solve() {
    input! {
        n: usize,
        sx: isize, sy: isize, tx: isize, ty: isize,
        xyrv: [(isize, isize, isize); n]
    };
    let sc = (sx, sy, 0);
    let tc = (tx, ty, 0_isize);

    let mut que = VecDeque::new();
    que.push_back(sc);
    let mut visited = HashSet::new();
    while let Some(p1) = que.pop_front() {
        eprintln!("{:?}", p1);
        if can_move(p1, tc) {
            println!("Yes");
            return;
        }
        for p2 in xyrv.iter().copied() {
            if visited.contains(&p2) {
                continue;
            }
            if can_move(p1, p2) {
                visited.insert(p2);
                que.push_back(p2);
            }
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

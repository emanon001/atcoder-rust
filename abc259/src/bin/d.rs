#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

type Circle = (isize, isize, isize);
pub fn is_inside_circle(c1: Circle, c2: Circle) -> bool {
    let dx = c1.0 - c2.0;
    let dy = c1.1 - c2.1;
    let dr = c1.2 - c2.2;
    dx * dx + dy * dy < dr * dr
}
pub fn is_outside_circle(c1: Circle, c2: Circle) -> bool {
    let dx = c1.0 - c2.0;
    let dy = c1.1 - c2.1;
    let dr = c1.2 + c2.2;
    dx * dx + dy * dy > dr * dr
}

fn can_move(from: Circle, to: Circle) -> bool {
    !is_inside_circle(from, to) && !is_outside_circle(from, to)
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

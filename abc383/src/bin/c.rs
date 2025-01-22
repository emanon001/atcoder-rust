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
        H: usize, W: usize, D: usize,
        S: [Chars; H],
    };

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == 'H' {
                heap.push((D, i, j));
                visited.insert((i, j));
            }
        }
    }
    while let Some((d, i, j)) = heap.pop() {
        if d == 0 {
            continue;
        }
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || nj < 0 || ni >= H as isize || nj >= W as isize {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if S[ni][nj] == '#' {
                continue;
            }
            if visited.contains(&(ni, nj)) {
                continue;
            }
            visited.insert((ni, nj));
            heap.push((d - 1, ni, nj));
        }
    }
    let ans = visited.len();
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

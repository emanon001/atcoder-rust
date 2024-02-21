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
        H: usize, W: usize, N: usize,
        T: Chars,
        grid: [Chars; H]
    };

    let mut ans = BTreeSet::new();
    for i in 0..H {
        for j in 0..W {
            if grid[i][j] == '#' {
                continue;
            }
            let mut ok = true;
            let mut pos_i = i;
            let mut pos_j = j;
            for &d in &T {
                let (di, dj) = match d {
                    'L' => (0, -1),
                    'R' => (0, 1),
                    'U' => (-1, 0),
                    'D' => (1, 0),
                    _ => unreachable!(),
                };
                let new_i = pos_i as isize + di;
                let new_j = pos_j as isize + dj;
                if new_i < 0 || new_i >= H as isize || new_j < 0 || new_j >= W as isize {
                    ok = false;
                    break;
                }
                pos_i = new_i as usize;
                pos_j = new_j as usize;
                if grid[pos_i][pos_j] == '#' {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans.insert((pos_i, pos_j));
            }
        }
    }
    println!("{}", ans.len());
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

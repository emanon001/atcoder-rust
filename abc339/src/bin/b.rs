#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize, N: usize,
    };

    let mut ans = vec![vec!['.'; W]; H];
    let mut pos = (0, 0);
    let mut dir_i = 0;
    for _ in 0..N {
        if ans[pos.0][pos.1] == '.' {
            ans[pos.0][pos.1] = '#';
            dir_i = (dir_i + 1) % DIRS.len();
        } else {
            ans[pos.0][pos.1] = '.';
            dir_i = if dir_i == 0 {
                DIRS.len() - 1
            } else {
                dir_i - 1
            };
        }
        let dir = DIRS[dir_i];
        let mut ni = pos.0 as isize + dir.0;
        if ni < 0 {
            ni = H as isize - 1;
        }
        if ni >= H as isize {
            ni = 0;
        }
        let mut nj = pos.1 as isize + dir.1;
        if nj < 0 {
            nj = W as isize - 1;
        }
        if nj >= W as isize {
            nj = 0;
        }
        pos = (ni as usize, nj as usize);
    }
    println!("{}", ans.iter().map(|row| row.iter().join("")).join("\n"));
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

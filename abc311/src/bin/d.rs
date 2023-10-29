#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        n: usize, m: usize,
        grid: [Chars; n]
    };

    let mut que = VecDeque::new();
    let mut visited = HashSet::new();
    let mut stops = HashSet::new();
    stops.insert((1_usize, 1_usize));
    que.push_back((1_usize, 1_usize));
    while let Some((i, j)) = que.pop_front() {
        visited.insert((i, j));
        // 各方向に岩にぶつかるまで進む
        for (di, dj) in &UDLR_DIRS {
            let mut cur_i = i as isize;
            let mut cur_j = j as isize;
            loop {
                let new_i = cur_i + di;
                let new_j = cur_j + dj;
                if new_i < 0 || new_i >= n as isize || new_j < 0 || new_j >= m as isize {
                    break;
                }
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                if grid[new_i][new_j] == '#' {
                    if !stops.contains(&(cur_i as usize, cur_j as usize)) {
                        stops.insert((cur_i as usize, cur_j as usize));
                        que.push_back((cur_i as usize, cur_j as usize));
                    }
                    break;
                }
                visited.insert((new_i, new_j));
                cur_i = new_i as isize;
                cur_j = new_j as isize;
            }
        }
    }
    println!("{}", visited.len());
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

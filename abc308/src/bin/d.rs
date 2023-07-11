#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h]
    };

    let mut visited = HashSet::new();
    let mut que = VecDeque::new();
    que.push_front((0_usize, 0_usize));
    while let Some((i, j)) = que.pop_front() {
        if i == h - 1 && j == w - 1 {
            println!("Yes");
            return;
        }
        for (di, dj) in &UDLR_DIRS {
            let new_i = i as isize + di;
            let new_j = j as isize + dj;
            if new_i < 0 || new_i >= h as isize || new_j < 0 || new_j >= w as isize {
                continue;
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            let new_ch = grid[new_i][new_j];
            let ok = match grid[i][j] {
                's' => new_ch == 'n',
                'n' => new_ch == 'u',
                'u' => new_ch == 'k',
                'k' => new_ch == 'e',
                'e' => new_ch == 's',
                _ => false,
            };
            let new_pos = (new_i, new_j);
            if ok && !visited.contains(&new_pos) {
                que.push_back(new_pos);
                visited.insert(new_pos);
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

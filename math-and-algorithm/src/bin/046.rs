#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        R: usize, C: usize,
        sy: Usize1, sx: Usize1,
        gy: Usize1, gx: Usize1,
        grid: [Chars; R],
    };

    let mut visited = HashSet::new();
    let mut que = VecDeque::new();
    que.push_back((sy, sx, 0));
    visited.insert((sy, sx));
    while let Some((y, x, d)) = que.pop_front() {
        if y == gy && x == gx {
            println!("{}", d);
            return;
        }
        for (dy, dx) in &UDLR_DIRS {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny < 0 || ny >= R as isize || nx < 0 || nx >= C as isize {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if grid[ny][nx] == '#' {
                continue;
            }
            if visited.contains(&(ny, nx)) {
                continue;
            }
            visited.insert((ny, nx));
            que.push_back((ny, nx, d + 1));
        }
    }
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

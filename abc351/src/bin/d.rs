#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize,
        Grid: [Chars; H]
    };

    let mut ans = 1;
    let mut visited = HashSet::new();
    for i in 0..H {
        for j in 0..W {
            if visited.contains(&(i, j)) {
                continue;
            }
            if Grid[i][j] == '#' {
                continue;
            }
            if is_touch_magnet(i, j, &Grid) {
                continue;
            }

            let count = traverse(i, j, &Grid, &mut visited);
            // eprintln!("{} {} = {}", i, j, count);
            // eprintln!("{:?}", visited);
            chmax!(ans, count);
        }
    }
    println!("{}", ans);
}

fn traverse(
    i: usize,
    j: usize,
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let mut res = 0;
    visited.insert((i, j));

    let mut visited_magnet = HashSet::new();

    let mut que = VecDeque::new();
    que.push_back((i, j));

    while let Some((i, j)) = que.pop_front() {
        res += 1;

        // 磁石が存在するなら処理を打ち切る
        if is_touch_magnet(i, j, grid) {
            continue;
        }

        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || ni >= grid.len() as isize || nj < 0 || nj >= grid[0].len() as isize {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if visited.contains(&(ni, nj)) || visited_magnet.contains(&(ni, nj)) {
                continue;
            }
            if grid[ni][nj] == '#' {
                continue;
            }
            if is_touch_magnet(ni, nj, grid) {
                visited_magnet.insert((ni, nj));
            } else {
                visited.insert((ni, nj));
            }
            que.push_back((ni, nj));
        }
    }
    res
}

fn is_touch_magnet(i: usize, j: usize, grid: &[Vec<char>]) -> bool {
    if i > 0 && grid[i - 1][j] == '#' {
        return true;
    }
    if i < grid.len() - 1 && grid[i + 1][j] == '#' {
        return true;
    }
    if j > 0 && grid[i][j - 1] == '#' {
        return true;
    }
    if j < grid[0].len() - 1 && grid[i][j + 1] == '#' {
        return true;
    }
    false
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

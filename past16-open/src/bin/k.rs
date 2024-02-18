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
        N: usize,
        G: [Chars; N],
    };
    let mut start = (0, 0);
    let mut goals = HashSet::new();
    for i in 0..N {
        for j in 0..N {
            match G[i][j] {
                'S' => start = (i, j),
                'G' => {
                    goals.insert((i, j));
                }
                _ => {}
            };
        }
    }
    let mut can_move_steps = vec![vec![vec![0; 4]; N]; N];
    // 上方向にどれだけ移動できるか
    for j in 0..N {
        let mut step = 0;
        for i in 0..N {
            can_move_steps[i][j][0] = step;
            if G[i][j] == 'X' {
                step = 0;
            } else {
                step += 1;
            }
        }
    }
    // 下方向にどれだけ移動できるか
    for j in 0..N {
        let mut step = 0;
        for i in (0..N).rev() {
            can_move_steps[i][j][1] = step;
            if G[i][j] == 'X' {
                step = 0;
            } else {
                step += 1;
            }
        }
    }
    // 左方向にどれだけ移動できるか
    for i in 0..N {
        let mut step = 0;
        for j in 0..N {
            can_move_steps[i][j][2] = step;
            if G[i][j] == 'X' {
                step = 0;
            } else {
                step += 1;
            }
        }
    }
    // 右方向にどれだけ移動できるか
    for i in 0..N {
        let mut step = 0;
        for j in (0..N).rev() {
            can_move_steps[i][j][3] = step;
            if G[i][j] == 'X' {
                step = 0;
            } else {
                step += 1;
            }
        }
    }

    // for i in 0..N {
    //     for j in 0..N {
    //         eprintln!(
    //             "({}, {}) => (U: {}, D: {}, L: {}, R: {})",
    //             i,
    //             j,
    //             can_move_steps[i][j][0],
    //             can_move_steps[i][j][1],
    //             can_move_steps[i][j][2],
    //             can_move_steps[i][j][3]
    //         )
    //     }
    // }

    for step in 1..N {
        let ans = shortest_path(start, &goals, &G, &can_move_steps, step);
        println!("{}", ans);
    }
}

fn udlr_dirs(step: usize) -> Vec<(isize, isize)> {
    let step = step as isize;
    vec![(-step, 0), (step, 0), (0, -step), (0, step)]
}

fn shortest_path(
    start: (usize, usize),
    goals: &HashSet<(usize, usize)>,
    grid: &[Vec<char>],
    can_move_steps: &[Vec<Vec<usize>>],
    step: usize,
) -> isize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut que = VecDeque::new();
    que.push_back((start, 0));
    while let Some(((i, j), cost)) = que.pop_front() {
        if goals.contains(&(i, j)) {
            return cost;
        }
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        for (k, (di, dj)) in udlr_dirs(step).iter().enumerate() {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || ni >= grid.len() as isize || nj < 0 || nj >= grid[0].len() as isize {
                continue;
            }
            if can_move_steps[i][j][k] < step {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if grid[ni][nj] == 'X' {
                continue;
            }
            if visited[ni][nj] {
                continue;
            }
            que.push_back(((ni, nj), cost + 1));
        }
    }
    -1
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

/*
3
S..
XG.
GX.
 */

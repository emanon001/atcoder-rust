#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use rand::prelude::*;
use std::time::{Instant, Duration};

struct Solver {
    n: usize,
    si: usize,
    sj: usize,
    grid: Vec<Vec<char>>,
    r: usize,
    score: f64,
    visited: HashSet<(usize, usize)>,
    path: Vec<char>,
    start_time: Instant,
    rng: ThreadRng,
}

impl Solver {
    fn new() -> Self {
        let now = Instant::now();
        input! {
            n: usize, si: usize, sj: usize,
            grid: [Chars; n]
        };
        let mut r = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] != '#' {
                    r += 1;
                }
            }
        }
        let path = Vec::new();
        Self {
            n,
            si,
            sj,
            grid,
            r,
            score: 0_f64,
            path,
            visited: HashSet::new(),
            start_time: now,
            rng: rand::thread_rng(),
        }
    }

    fn solve(&mut self) {
        self.visited.insert((self.si, self.sj));
        let pos = self.move_vertical(self.si, self.sj);
        if pos != (self.si, self.sj) {
            let rest_path = self.shortest_path(pos, (self.si, self.sj));
            println!("{}{}", self.path.iter().join(""), rest_path.iter().join(""));
            return;
        }

        let pos = self.move_horizontal(self.si, self.sj);
        if pos != (self.si, self.sj) {
            let rest_path = self.shortest_path(pos, (self.si, self.sj));
            println!("{}{}", self.path.iter().join(""), rest_path.iter().join(""));
            return;
        }
        println!("{}", self.path.iter().join(""));
    }

    fn move_vertical(&mut self, i: usize, j: usize) -> (usize, usize) {
        // 上
        if i > 0 {
            let mut step = 0;
            for move_i in (0..i).rev() {
                if self.grid[move_i][j] == '#' {
                    break;
                }
                if self.visited.contains(&(move_i, j)) {
                    break;
                }
                self.visited.insert((move_i, j));
                step += 1;
                self.path.push('U');
                if self.visited.len() == self.r {
                    // 全てのマスを見た
                    return (move_i, j);
                }
                let pos = self.move_horizontal(move_i, j);
                if pos != (move_i, j) {
                    return pos;
                }
            }
            for _ in 0..step {
                self.path.push('D');
            }
        }
        // 下
        if i < self.n - 1 {
            let mut step = 0;
            for move_i in i + 1..self.n {
                if self.grid[move_i][j] == '#' {
                    break;
                }
                if self.visited.contains(&(move_i, j)) {
                    break;
                }
                self.visited.insert((move_i, j));
                step += 1;
                self.path.push('D');
                if self.visited.len() == self.r {
                    // 全てのマスを見た
                    return (move_i, j);
                }
                let pos = self.move_horizontal(move_i, j);
                if pos != (move_i, j) {
                    return pos;
                }
            }
            for _ in 0..step {
                self.path.push('U');
            }
        }
        (i, j)
    }

    fn move_horizontal(&mut self, i: usize, j: usize) -> (usize, usize) {
        // 左
        if j > 0 {
            let mut step = 0;
            for move_j in (0..j).rev() {
                if self.grid[i][move_j] == '#' {
                    break;
                }
                if self.visited.contains(&(i, move_j)) {
                    break;
                }
                self.visited.insert((i, move_j));
                step += 1;
                self.path.push('L');
                if self.visited.len() == self.r {
                    // 全てのマスを見た
                    return (i, move_j);
                }
                let pos = self.move_vertical(i, move_j);
                if pos != (i, move_j) {
                    return pos;
                }
            }
            // 戻り
            for _ in 0..step {
                self.path.push('R');
            }
        }
        // 右
        if j < self.n - 1 {
            let mut step = 0;
            for move_j in j + 1..self.n {
                if self.grid[i][move_j] == '#' {
                    break;
                }
                if self.visited.contains(&(i, move_j)) {
                    break;
                }
                self.visited.insert((i, move_j));
                step += 1;
                self.path.push('R');
                if self.visited.len() == self.r {
                    // 全てのマスを見た
                    return (i, move_j);
                }
                let pos = self.move_vertical(i, move_j);
                if pos != (i, move_j) {
                    return pos;
                }
            }
            for _ in 0..step {
                self.path.push('L');
            }
        }
        (i, j)
    }

    fn shortest_path(&self, goal: (usize, usize), start: (usize, usize)) -> Vec<char> {
        let mut cost_map: HashMap<(usize, usize), i64> = HashMap::new();
        let mut heap = std::collections::BinaryHeap::new();
        cost_map.insert(goal, 0_i64);
        heap.push(std::cmp::Reverse((0_i64, goal)));
        let mut path_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        while let Some(std::cmp::Reverse((cost, pos))) = heap.pop() {
            if cost_map.contains_key(&pos) && cost > *cost_map.get(&pos).unwrap() {
                continue;
            }
            let i = pos.0;
            let j = pos.1;
            let directions = vec![
                (0, -1),
                (1, 0),
                (0, 1),
                (-1, 0),
            ];
            for (di, dj) in directions {
                let new_i = i as isize + di;
                let new_j = j as isize + dj;
                if new_i < 0 || new_i >= self.n as isize || new_j < 0 || new_j >= self.n as isize {
                    continue;
                }
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                if self.grid[new_i][new_j] == '#' {
                    continue;
                }
                let new_cost = cost + self.grid[new_i][new_j].to_digit(10).unwrap() as i64;
                let new_pos = (new_i, new_j);
                if !cost_map.contains_key(&new_pos) || new_cost < cost_map[&new_pos] {
                    heap.push(std::cmp::Reverse((new_cost, new_pos)));
                    cost_map.insert(new_pos, new_cost);
                    path_map.insert(new_pos, pos);
                }
            }
        }
        let mut path = VecDeque::new();
        let mut to_pos = start;
        while to_pos != goal {
            let from_pos = path_map[&to_pos];
            // eprintln!("{:?} {:?}", to_pos, from_pos);
            let d = if from_pos.0 + 1 == to_pos.0 {
                'D'
            } else if from_pos.0 == to_pos.0 + 1 {
                'U'
            } else if from_pos.1 + 1 == to_pos.1 {
                'R'
            } else {
                'L'
            };
            path.push_front(d);
            to_pos = from_pos;
        }
        // eprintln!("goal: {:?}, start: {:?}", goal, start);
        // eprintln!("path: {}", path.iter().join(""));
        path.into_iter().collect::<Vec<_>>()
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}

use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashSet, VecDeque};

fn shortest_path(
  grid: &Vec<Vec<char>>,
  h: usize,
  w: usize,
  start: (usize, usize),
  goal: (usize, usize),
) -> usize {
  let di = vec![-1, 0, 1, 0];
  let dj = vec![0, 1, 0, -1];
  let mut visited = HashSet::new();
  let mut que = VecDeque::new();
  que.push_back((start, 0));
  while let Some(((i, j), cost)) = que.pop_front() {
    for d in 0..4 {
      let new_i = (i as isize) + di[d];
      let new_j = (j as isize) + dj[d];
      if new_i < 0 || new_i >= (h as isize) || new_j < 0 || new_j >= (w as isize) {
        continue;
      }
      let new_i = new_i as usize;
      let new_j = new_j as usize;
      if grid[new_i][new_j] == 'X' {
        continue;
      }
      let new_pos = (new_i, new_j);
      if visited.contains(&new_pos) {
        continue;
      }
      visited.insert(new_pos);
      let new_cost = cost + 1;
      if new_pos == goal {
        return new_cost;
      }
      que.push_back((new_pos, new_cost));
    }
  }
  panic!();
}

fn main() {
  input! {
    h: usize, w: usize, n: usize,
    grid: [Chars; h]
  };

  let mut pos = vec![(0, 0); n + 1];
  for i in 0..h {
    for j in 0..w {
      match grid[i][j] {
        'S' => pos[0] = (i, j),
        ch => {
          if let Some(n) = ch.to_digit(10) {
            pos[n as usize] = (i, j);
          }
        }
      };
    }
  }
  let mut res = 0;
  for i in 0..n {
    res += shortest_path(&grid, h, w, pos[i], pos[i + 1]);
  }
  println!("{}", res);
}

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
  input! {
    r: usize, c: usize,
    s: (Usize1, Usize1),
    g: (Usize1, Usize1),
    grid: [Chars; r]
  };

  let di = vec![-1, 0, 1, 0];
  let dj = vec![0, 1, 0, -1];
  let mut visited = HashSet::new();
  let mut que = VecDeque::new();
  que.push_back((s, 0));
  while let Some(((i, j), cost)) = que.pop_front() {
    for d in 0..4 {
      let new_i = (i as isize) + di[d];
      let new_j = (j as isize) + dj[d];
      if new_i < 0 || new_i >= (r as isize) || new_j < 0 || new_j >= (c as isize) {
        continue;
      }
      let new_i = new_i as usize;
      let new_j = new_j as usize;
      if grid[new_i][new_j] == '#' {
        continue;
      }
      let new_pos = (new_i, new_j);
      if visited.contains(&new_pos) {
        continue;
      }
      visited.insert(new_pos);
      let new_cost = cost + 1;
      if new_pos == g {
        println!("{}", new_cost);
        std::process::exit(0);
      }
      que.push_back((new_pos, new_cost));
    }
  }
}

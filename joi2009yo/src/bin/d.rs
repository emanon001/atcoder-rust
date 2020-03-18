use proconio::input;
use std::collections::HashSet;

fn longest_path(
  i: usize,
  j: usize,
  grid: &Vec<Vec<usize>>,
  n: usize,
  m: usize,
  visited: &mut HashSet<(usize, usize)>,
) -> usize {
  let dy = vec![-1, 0, 1, 0];
  let dx = vec![0, 1, 0, -1];
  let mut res = 1;
  for d in 0..4 {
    let y = (i as isize) + dy[d];
    let x = (j as isize) + dx[d];
    if y < 0 || y >= (m as isize) || x < 0 || x >= (n as isize) {
      continue;
    }
    if grid[y as usize][x as usize] == 0 {
      continue;
    }
    let pos = (y as usize, x as usize);
    if visited.contains(&pos) {
      continue;
    }
    visited.insert(pos);
    let n = longest_path(y as usize, x as usize, grid, n, m, visited) + 1;
    if n > res {
      res = n;
    }
    visited.remove(&pos);
  }
  res
}

fn main() {
  input! {
    n: usize,
    m: usize,
    grid: [[usize; n]; m]
  };

  let mut res = 0;
  for i in 0..m {
    for j in 0..n {
      if grid[i][j] == 0 {
        continue;
      }
      let mut visited = HashSet::new();
      visited.insert((i, j));
      let n = longest_path(i, j, &grid, n, m, &mut visited);
      if n > res {
        res = n;
      }
    }
  }
  println!("{}", res);
}

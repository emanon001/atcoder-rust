use proconio::input;
use proconio::marker::Chars;

pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
  F: Fn(i64) -> bool,
{
  let orig_ok = ok;
  let mut ok = ok;
  let mut ng = ng;
  while (ok - ng).abs() > 1 {
    let mid = (ok + ng) / 2;
    if pred(mid) {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  if ok == orig_ok {
    None
  } else {
    Some(ok)
  }
}

fn shortest_path(
  grid: &Vec<Vec<char>>,
  h: usize,
  w: usize,
  start: (usize, usize),
  goal: (usize, usize),
  x: u64,
) -> u64 {
  let inf = 1_u64 << 60;
  let di = vec![-1, 0, 1, 0];
  let dj = vec![0, 1, 0, -1];
  let mut cost_list = vec![vec![inf; w]; h];
  cost_list[start.0][start.1] = 0;
  let mut que = std::collections::VecDeque::new();
  que.push_back((start, 0_u64));
  while let Some(((i, j), cost)) = que.pop_front() {
    if cost_list[i][j] < cost {
      continue;
    }
    for d in 0..4 {
      let new_i = (i as isize) + di[d];
      let new_j = (j as isize) + dj[d];
      if new_i < 0 || new_i >= (h as isize) || new_j < 0 || new_j >= (w as isize) {
        continue;
      }
      let new_i = new_i as usize;
      let new_j = new_j as usize;
      let new_pos = (new_i, new_j);
      let new_cost = cost
        + match grid[new_pos.0][new_pos.1] {
          '.' => 1,
          'S' => 1,
          'G' => 1,
          '#' => x,
          _ => panic!(),
        };
      if new_cost < cost_list[new_pos.0][new_pos.1] {
        cost_list[new_pos.0][new_pos.1] = new_cost;
        que.push_back((new_pos, new_cost));
      }
    }
  }
  cost_list[goal.0][goal.1]
}

fn main() {
  input! {
    h: usize, w: usize, t: u64,
    grid: [Chars; h]
  };

  let mut s = (0_usize, 0_usize);
  let mut g = (0_usize, 0_usize);
  for i in 0..h {
    for j in 0..w {
      let cell = grid[i][j];
      match cell {
        'S' => s = (i, j),
        'G' => g = (i, j),
        _ => {}
      }
    }
  }

  let res = bsearch(0, t as i64, |x| {
    let cost = shortest_path(&grid, h, w, s, g, x as u64);
    cost <= t
  });
  println!("{}", res.unwrap());
}

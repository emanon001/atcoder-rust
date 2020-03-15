use proconio::input;
use std::collections::HashSet;

fn main() {
  input! {
    m: usize,
    coordinates: [(i32, i32); m],
    n: usize,
    p_coordinates: [(i32, i32); n],
  };

  let mut p_set = HashSet::new();
  for (x, y) in &p_coordinates {
    p_set.insert((*x, *y));
  }
  let base_coord = coordinates[0];
  for (px, py) in p_coordinates {
    let x_diff = px - base_coord.0;
    let y_diff = py - base_coord.1;
    let is_ok = coordinates.iter().all(|(x, y)| {
      let coord = (x + x_diff, y + y_diff);
      p_set.contains(&coord)
    });
    if is_ok {
      println!("{} {}", x_diff, y_diff);
    }
  }
}

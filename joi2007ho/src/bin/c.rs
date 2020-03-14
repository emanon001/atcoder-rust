use proconio::input;
use std::collections::HashSet;

fn rest_coordinates(a: Coordinate, b: Coordinate) -> (Coordinate, Coordinate) {
  let c = (b.0 - b.1 + a.1, b.1 + b.0 - a.0);
  let d = (c.0 - c.1 + b.1, c.1 + c.0 - b.0);
  (c, d)
}

type Coordinate = (i32, i32);

fn main() {
  input! {
    n: usize,
    coordinates: [(i32, i32); n]
  };

  let mut set: HashSet<Coordinate> = HashSet::new();
  for c in &coordinates {
    set.insert(*c);
  }
  let mut res = 0;
  for i in 0..(n - 1) {
    let a = coordinates[i];
    for j in i..n {
      let b = coordinates[j];
      let v = vec![rest_coordinates(a, b), rest_coordinates(b, a)];
      for (c, d) in v {
        if set.contains(&c) && set.contains(&d) {
          let area = (c.0 - d.0).abs().pow(2) + (c.1 - d.1).abs().pow(2);
          if area > res {
            res = area
          }
        }
      }
    }
  }
  println!("{}", res);
}

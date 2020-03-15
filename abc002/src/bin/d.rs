use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;

fn main() {
  input! {
    n: usize, m: usize,
    xy_list: [(Usize1, Usize1); m]
  };

  let mut relations = Vec::new();
  for _ in 0..n {
    relations.push(HashSet::new());
  }
  for (x, y) in xy_list {
    relations[x].insert(y);
    relations[y].insert(x);
  }
  let mut res = 0;
  for bits in 0..(1 << n) {
    let is_ok = (0..n).all(|i| {
      if ((bits & (1 << i)) >> i) == 0 {
        return true;
      }
      (0..n).all(|j| {
        if i == j {
          return true;
        }
        if ((bits & (1 << j)) >> j) == 0 {
          return true;
        }
        relations[i].contains(&j)
      })
    });
    if is_ok {
      let count = format!("{:b}", bits)
        .chars()
        .filter(|ch| ch == &'1')
        .count();
      if count > res {
        res = count;
      }
    }
  }
  println!("{}", res);
}
